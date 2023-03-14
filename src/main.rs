use std::{
    str,
    io::prelude::*,
    net::TcpStream
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2088").unwrap();
    let mut read = [0; 128];
    stream.read(&mut read);
    println!("> {}", str::from_utf8(&read).unwrap());

    for i in 1..200 {
        let cmd = format!("set {} in _basedb to yepthisisitdef", i);
        println!("{cmd}");
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        println!("> {}", str::from_utf8(&read).unwrap());
    }
}
