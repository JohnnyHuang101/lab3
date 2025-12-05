//refactored script_gen.rs. Provides the grab_trimmed_file_lines used by other rs files to read in lines from a speak file. Aman Verma, Johnny Huang, Hanson Li

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use super::declarations::GENERATION_FAILURE;

fn get_buffered_reader(file_name: &String) -> io::Result<BufReader<Box<dyn Read>>> {
    if file_name.starts_with("net:") {
        let rest = &file_name[4..];

        let mut parts = rest.splitn(3, ':');
        let ip = parts.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing IP in net: path")
        })?;
        let port = parts.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing port in net: path")
        })?;
        let filename = parts.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing filename in net: path")
        })?;

        let addr = format!("{ip}:{port}");

        let mut stream = TcpStream::connect(addr)?;

        // match lab3testclient: send token bytes, no newline
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
            let _ = writeln!(stdout,"Error: in grab_trimmed_file_lines, failed to open file with error code: {} and file name: {}", e_code, file_name);
            return Err(GENERATION_FAILURE);
        }
    }
}
