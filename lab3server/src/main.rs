//main.rs in lab3server init a new Server instance. Johnny Huang, Aman Verma, Hanson Li
pub mod lab3;// look inside lab3 folder

use lab3::server::Server;
use lab3::return_wrapper::ReturnWrapper;
use std::env;
use lab3::declarations::{
    ARGS_MIN,
    EXIT_BAD_CMDLINE,
    SUCCESS_CODE,
    ARG_PROGRAM_IDX,
    CONNECTION_ERR
};


pub fn main() -> ReturnWrapper {
    let args: Vec<String> = env::args().collect();

    if args.len() != ARGS_MIN {
        eprintln!("Usage is >>Program name<< {} <address:port>", args[ARG_PROGRAM_IDX]);
        return ReturnWrapper::new(EXIT_BAD_CMDLINE);// usage error
    }

    let address = &args[1];

    let mut server = Server::new();

    if let Err(e) = server.open(address) {
        eprintln!("failed to open server on adress {}: {}", address, e);
        return ReturnWrapper::new(CONNECTION_ERR);
    }

    println!("Address successfully found and opened! Running Server...");
    server.run();

    ReturnWrapper::new(SUCCESS_CODE) 
}
