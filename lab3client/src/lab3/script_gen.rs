//refactored script_gen.rs. Provides the grab_trimmed_file_lines used by other rs files to read in lines from a speak file. Aman Verma, Johnny Huang, Hanson Li

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use super::declarations::{GENERATION_FAILURE, NETFILE_IDX, NETFILE_COMPONENT_NUM};
use std::net::IpAddr; //for checking valid ip address

fn get_buffered_reader(file_name: &String) -> io::Result<BufReader<Box<dyn Read>>> {
    if file_name.starts_with("net:") {
        let fname_after_net = &file_name[NETFILE_IDX..]; //pos 4 in file_name is the ip addr followed by actual fname

        let fname_parts: Vec<&str> = fname_after_net.split(":").collect();

        //if num of componenets !=3, return an error msg
        if fname_parts.len() != NETFILE_COMPONENT_NUM {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "remote filename incorrect. Need to be in format net:<ip addr>:<port number>:<filename>."));
        }
        
        //check if this is a valid ip address: https://doc.rust-lang.org/std/net/enum.IpAddr.html
        if !fname_parts[0].parse::<IpAddr>().is_ok(){
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "first component after net: is not a valid IP address. Must be ipv4 or ipv6."))
        };
        
        let ip = fname_parts[0].clone();
        let port = fname_parts[1].clone();
        let filename = fname_parts[2].clone();

        let addr = format!("{ip}:{port}");

        let mut stream = TcpStream::connect(addr)?;

        //match lab3testclient - send token bytes, no newline
        stream.write_all(filename.as_bytes())?;
        stream.flush()?;

        Ok(BufReader::new(Box::new(stream) as Box<dyn Read>))
    } else {
        let file = File::open(file_name)?;
        Ok(BufReader::new(Box::new(file) as Box<dyn Read>))
    }
}

pub fn grab_trimmed_file_lines(file_name: &String, file_line_vec: &mut Vec<String>) -> Result<(), u8>{
    //note: in config files you must provide the relative or full path to the speak files
    //using match since error code could be helpful here

    let mut stdout = io::stdout().lock();
    println!("calling get buffered reader");
    match get_buffered_reader(file_name) {
        Ok(mut buf_reader) => {
            let mut cur_read_str = String::new();

            loop {
 
                cur_read_str.clear(); //clear first, then read_line
                match buf_reader.read_line(&mut cur_read_str) {
                    Ok(read_line_status) => {
                        
                        if read_line_status == 0 {
                            break
                        }

                        file_line_vec.push(cur_read_str.trim().to_string()); //trim will return &str so we need to_string
                    },
                    Err(e_code) => {
                        let _ = writeln!(stdout,"Error: in grab_trimmed_file_lines, BufReader failed with code {}", e_code);
                        return Err(GENERATION_FAILURE);
                    },
                    
                }

            }
            
            Ok(()) //only arrive here if we break from the loop

        }
        Err(e_code) => {
            let _ = writeln!(stdout,"Error: in grab_trimmed_file_lines, failed to open file with error details: {} your file name: {}", e_code, file_name);
            return Err(GENERATION_FAILURE);
        }
    }
}
