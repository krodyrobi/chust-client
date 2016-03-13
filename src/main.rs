extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

mod connection;

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use rustc_serialize::json;

use connection::{ClientRequest, Response};

const USAGE: &'static str = "
Chust client.

Usage:
  chust_client register <username> <pass>
  chust_client login <username> <pass>

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_username: String,
    arg_pass: String,
    cmd_register: bool,
    cmd_login: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let stream = TcpStream::connect("127.0.0.1:25658").unwrap();
    let mut writestr = stream.try_clone().unwrap();
    stream.set_read_timeout(Some(Duration::new(0, 100000))).unwrap();

    if args.cmd_register {
        let reg = ClientRequest::Reg(args.arg_username.clone(), args.arg_pass.clone());
        let line = format!("{}\n", json::encode(&reg).unwrap());

        writestr.write(line.as_bytes()).unwrap();
    }

    if args.cmd_login {
        let auth = ClientRequest::Auth(args.arg_username.clone(), args.arg_pass.clone());
        let line = format!("{}\n", json::encode(&auth).unwrap());

        writestr.write(line.as_bytes()).unwrap();
    }

    thread::spawn(move || {
        let mut reader = BufReader::new(stream);

        loop {
            let mut line = String::new();
            let mut flag = false;
            match reader.read_line(&mut line) {
                Ok(_) => {
                    match json::decode(&line) {
                        Ok(x) => {
                            match x {
                                Response::Ok => {
                                    if args.cmd_register.clone() {
                                        println!("Registered successfully");
                                        std::process::exit(0);
                                    }
                                    // println!("Got ok");
                                }
                                Response::Err(code, s) => {
                                    println!("Got server err {}", s);
                                    if code == 2 || code == 3 || code == 4 {
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            println!("{}", line.trim_right());
                        }
                    }
                }
                Err(_) => {
                    flag = true;
                }
            }
            if flag {
                thread::sleep(Duration::new(0, 300000));
            }
        }
    });

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim_right().to_string();
        if line == "/exit" {
            drop(writestr);
            std::process::exit(1);
        }

        let send = ClientRequest::Send(line);
        let line = format!("{}\n", json::encode(&send).unwrap());

        writestr.write(line.as_bytes()).unwrap();
    }
}
