//simple client that tests connection to server and see if it can server files. Aman Verma, Hanson Li, Johnny Huang

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
        process::exit(1);
    }

    let address = &args[1];
    let token = &args[2];

    let mut stream = TcpStream::connect(address)?; //should be passed in as a copy here?

    stream.write_all(token.as_bytes())?;

    if token != "quit" {
        let reader = BufReader::new(&stream);

        //this will run continously until conenction is shtudown by server
        for line in reader.lines() {
            match line {
                Ok(text) => println!("{}", text),
                Err(e) => eprintln!("Error reading from server: {}", e),
            }
        }
    } else {
        let sleep_len = Duration::from_secs(1);
        
        thread::sleep(sleep_len);

        //CONNecting to wake up the server again. the server needs to be waken up before it listens to commands
        let _ = TcpStream::connect(address)?;
    }

    Ok(())
}