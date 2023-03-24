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

    let testlen = 20000;
    print!("<");

    for i in 1..testlen {
        let cmd = format!("get {} from _basedb\n", i);
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        
        if i % 500 == 0 {
            println!("{}\t / {:?}", i, str::from_utf8(&read).unwrap_or("-").trim_matches(char::from(0)));
        }
    }
}
