use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage is {} <address:port> <token>", args[0]);
        process::exit(1); // Return an error code
    }

    let address = &args[1];
    let token = &args[2];

    let mut stream = TcpStream::connect(address)?;

    stream.write_all(token.as_bytes())?;

    if token != "quit" {
        // Create a buffered reader to read lines of text
        let reader = BufReader::new(&stream);

        // Read lines until the connection is shut down by the server
        for line in reader.lines() {
            match line {
                Ok(text) => println!("{}", text),
                Err(e) => eprintln!("Error reading from server: {}", e),
            }
        }
        // Once the loop finishes (server closes stream)
    } else {
        let pause = Duration::from_secs(1);
        
        thread::sleep(pause);

        //CONNecting to wake up the server again. the server needs to be waken up before it listens to commands
        let _ = TcpStream::connect(address)?;
    }

    Ok(())
}