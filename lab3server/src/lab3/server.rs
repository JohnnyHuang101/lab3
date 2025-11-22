use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

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

            //if accept succeeded, spawn child thread
            if let Ok((socket, _addr)) = incoming {
                thread::spawn(move || {
                    use std::io::{Read, Write};
                    use std::fs::File;
                    use std::sync::atomic::Ordering;
                    let mut stream = socket;

                    //read a single token from the stream
                    let mut buffer = [0u8; 1024];
                    let bytes_read = match stream.read(&mut buffer) {
                        Ok(n) => n,
                        Err(_) => return, // connection error → just exit
                    };

                    if bytes_read == 0 {
                        return; // nothing is send, client not responsive
                    }

                    //convert token to string
                    let token = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

                    // Handle "quit"
                    if token == "quit" {
                        println!("shutting down server...");
                        CANCEL.store(true, Ordering::SeqCst);
                        return;
                    }

                    //secuirty check to not allow file browsing. the only file should live in server.rs
                    if token.contains('/') || token.contains('\\') ||
                    token.contains("..") || token.contains('$') {
                        let _ = stream.write_all(b"Invalid filename. Acces snot granted\n");
                        return;
                    }

                    // Try opening file
                    println!("NEW FILE REQUEST, trying to open it up...");
                    let mut file = match File::open(&token) {
                        Ok(f) => f,
                        Err(_) => {
                            let msg = format!("Could not open the file {}.\n", token);// have to formati it like this or cant stream
                            let _ = stream.write_all(msg.as_bytes());
                            return;
                        }
                    };

                    //after fiole is successfully read then we are going to return it
                    let mut file_buf = Vec::new();
                    if file.read_to_end(&mut file_buf).is_ok() {
                        let _ = stream.write_all(&file_buf);
                    }

                    let _ = stream.shutdown(std::net::Shutdown::Both);
                });
            }
        }
    }
}
