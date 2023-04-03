use std::{
    str,
    ops::Rem,
    fs::File,
    time::Instant,
    io,
    io::{prelude::*, BufReader, Write},
    net::TcpStream
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2088").unwrap();
    let mut buffer = BufReader::new(stream.try_clone().unwrap());
    let mut output = File::create("./output.dat").unwrap();
    
    let mut read = [0; 128];
    stream.read(&mut read);
    println!("Starting to write test data to DB now...");
    println!("{}", str::from_utf8(&read).unwrap().trim_matches(char::from(0)));

    let mut test: (f64, usize) = (0.0, 0);

    let testlen = 10000;
    let persymbol = (testlen / 100, testlen / 100 - 1);

    let timestart = Instant::now();

    print!("<");
    io::stdout().flush().unwrap();
    for i in 1..testlen {
        let cmd = format!("set {i} in _basedb to testing\n");
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        
        if i % persymbol.0 == persymbol.1 {
            print!("=");
            io::stdout().flush().unwrap();
        }
    }
    print!(">");
    io::stdout().flush().unwrap();

    println!("\nCompleted writing in {} seconds\nStarting now to read...", timestart.elapsed().as_secs());
    let timestart = Instant::now();

    print!("<");
    io::stdout().flush().unwrap();
    for i in 1..testlen {
        let cmd = format!("get {i} from _basedb\n");
        stream.write(cmd.as_bytes());
        let mut read = [0; 128];
        stream.read(&mut read);
        let mut read2 = [0; 40];
        stream.read(&mut read2);
        
        if i % 5 == 4 {
            let res = str::from_utf8(&read).unwrap_or("-").trim_matches(char::from(0));
            let res2 = str::from_utf8(&read2).unwrap_or("-").trim_matches(char::from(0));
            // println!("L: {}; {}", res, &res2[1..13]);
            if res2.len() >= 19 {
                test = (&res2[1..13].trim().parse::<f64>().unwrap_or(0.0) + test.0, test.1 + 1);
                let out = format!("{:8?}{}{}{}", i, "\t", &res2[1..13].trim(), "\n");
                output.write(out.as_bytes());
            }

            if i % persymbol.0 == persymbol.1 {
                print!("=");
                io::stdout().flush().unwrap();
            }
        }
    }
    print!(">");
    io::stdout().flush().unwrap();

    println!("\nCompleted reading in {} seconds", timestart.elapsed().as_secs());
    println!("AVG query processing time: {:.2?} nano seconds", test.0 / test.1 as f64);
}
