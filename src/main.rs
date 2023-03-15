use std::{
    str,
    ops::Rem,
    io::{prelude::*, BufReader},
    net::TcpStream
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2088").unwrap();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());
    // let mut read = String::from("");
    // buffer.read(&mut read);
    // println!("> {:?}", str::from_utf8(&read).unwrap());

    let testlen = 100000;
    print!("<");

    for i in 1..testlen {
        let cmd = format!("set {} in _basedb to yepthisisitdef\n", i);
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        
        if i % 1000 == 0 {
            println!("{} / {:?}", i, str::from_utf8(&read).unwrap_or("-").trim_matches(char::from(0)));
        }
    }
}
