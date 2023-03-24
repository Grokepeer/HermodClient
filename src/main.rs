use std::{
    str,
    ops::Rem,
    io::{prelude::*, BufReader},
    net::TcpStream
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2088").unwrap();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());

    let mut read = [0; 128];
    stream.read(&mut read);
    println!("{:?}", str::from_utf8(&read).unwrap().trim_matches(char::from(0)));

    let mut test: (f64, usize) = (0.0, 0);

    let testlen = 100000; //

    for i in 1..testlen {
        let cmd = format!("get _base from _basedb\n");
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        
        if i % 10 == 0 {
            let res = str::from_utf8(&read).unwrap_or("-").trim_matches(char::from(0));
            if res.len() >= 5 {
                test = (&res[1..6].trim().parse::<f64>().unwrap_or(0.0) + test.0, test.1 + 1);
            }
        }
    }

    println!("AVG query processing time: {:.2?} nano seconds", test.0 / test.1 as f64);
}
