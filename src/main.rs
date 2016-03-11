extern crate rustc_serialize;

mod connection;

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use rustc_serialize::json::{self, Json};

use connection::{ClientRequest, ServerRequest, Response};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:25658").unwrap();

    let reg = ClientRequest::Reg("new".to_string(), "user".to_string());
    let line = format!("{}\n", json::encode(&reg).unwrap());

    stream.write(line.as_bytes()).unwrap();

    let auth = ClientRequest::Auth("new".to_string(), "user".to_string(), "25659".to_string());
    let line = format!("{}\n", json::encode(&auth).unwrap());

    stream.write(line.as_bytes()).unwrap();

    loop {
        let mut reader = BufReader::new(&stream);

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let send = ClientRequest::Send(line.trim_right().to_string());
        let line = format!("{}\n", json::encode(&send).unwrap());

        reader.get_mut().write(line.as_bytes()).unwrap();

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        println!("{}", line);
    }
}
