use std::net::TcpStream;
use std::io::Write;
use std::io::BufRead;
use std::io::{BufReader, BufWriter};

fn main() {
    let host = "whois.radb.net";
    let port = 43;
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(stream) => {
            println!("connected to {}:{}", host, port);
            let mut reader = BufReader::new(&stream);
            let mut writer = BufWriter::new(&stream);

            let msg = format!("AS-GOOGLE\n");
            let _ = writer.write(msg.as_bytes());
            writer.flush().unwrap();

            loop {
                let mut line = String::new();
                let bytes = reader.read_line(&mut line).unwrap();
                if bytes == 0 {
                    break;
                }
                println!("{}", line.trim_end());
            }
        },
        Err(e) => {
            println!("connection error: {}", e);
        }
    }
}
