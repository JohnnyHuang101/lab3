//server.rs binds TCP listner, provides run method that spawns child threads for manage new socket conns. Aman Verma, Hanson Li, Johnny Huang
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::io::{Read, Write, BufReader, BufWriter};
use std::fs::File;

//a static cancellation flag initialized to false so tat its not true right now
pub static CANCEL: AtomicBool = AtomicBool::new(false);

pub struct Server {
    pub listener: Option<TcpListener>,
    pub listening_addr: String,
}

impl Server {
    ///ssociated constructor
    pub fn new() -> Self {
        Self {
            listener: None,
            listening_addr: String::new(),
        }
    }

    ///returns true if the server currently has an open listener
    pub fn is_open(&self) -> bool {
        self.listener.is_some()
    }

    ///going to attempts to open a listener on the given address
    pub fn open(&mut self, address: &str) -> std::io::Result<()> {
        match TcpListener::bind(address) {// binds the adress
            Ok(l) => {
                self.listener = Some(l);
                self.listening_addr = address.to_string();//not mandatry
                Ok(())
            }
            Err(e) => Err(e),
        }
    }



    pub fn run(&self) {
        //only going run if the server is open, safety chejck
        if self.listener.is_none() {
            return;
        }

        //saafe to unwrap because we checked above
        let listener = self.listener.as_ref().unwrap();

        //loop until cancellation requested or listener is gone
        while !CANCEL.load(Ordering::Relaxed) && self.is_open() {
            //attempt to accept a new connection
            let incoming = listener.accept();
            
            if CANCEL.load(Ordering::Relaxed) {
                return;
            }

            //spawn child thread from listner
            if let Ok((socket, _addr)) = incoming {
                thread::spawn(move || {

                    //currently don't need bufreader/writers but adding them in for flexibility for poentital test cases
                    let mut stream_bufreader = BufReader::new(&socket);
                    let mut stream_bufwriter = BufWriter::new(&socket);

                    //read a single token from the stream
                    let mut buffer = [0u8; 1024];
                    let bytes_read = match stream_bufreader.read(&mut buffer) { //we should probably replace read with read_line in the future so we don't have to manage a buffer
                        Ok(n) => n,
                        Err(_) => return,
                    };

                    if bytes_read == 0 {
                        println!("0 bytes read, shutting down server");
                        return; // nothing is send, client not responsive
                    }

                    //tok2str, using lossy version since it is less strict, we can let our conditions downstream check for validity
                    let token = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

                    //handle "quit"
                    if token == "quit" {
                        println!("shutting down server!");
                        CANCEL.store(true, Ordering::SeqCst);
                        return;
                    }

                    //secuirty check to not allow file browsing. the only file should live in lab3server folder
                    if token.contains("..") || token.contains('/') || token.contains('\\') || token.contains('$') {
                        let _ = stream_bufwriter.write_all(b"Invalid filename. cannot contain slashes, $, or '..'. Access not granted!\n");
                        let _ = stream_bufwriter.flush();
                        return;
                    }

                    //opening file from server side, then stream contents of file to client if success else stream a failure msg
                    println!("NEW FILE REQUEST, trying to open it up...");
                    let mut file = match File::open(&token) {
                        Ok(f) => f,
                        Err(e_code) => {
                            let msg = format!("Could not open the file {}\n with err code: {}\n", token, e_code);// have to formati it like this or cant stream
                            let _ = stream_bufwriter.write_all(msg.as_bytes());
                            let _ = stream_bufwriter.flush(); //need to flush otherwise client wont get msg
                            return;
                        }
                    };
                    
                    //after fiole is successfully read then we are going to return it
                    let mut file_buf = Vec::new();
                    if file.read_to_end(&mut file_buf).is_ok() {
                        let _ = stream_bufwriter.write_all(&file_buf);
                        let _ = stream_bufwriter.flush();
                    }
                    
                    let _ = socket.shutdown(std::net::Shutdown::Both);
                });
            }
        }
    }
}
