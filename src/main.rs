use std::{
    str,
    thread,
    time,
    // ops::Rem,
    fs::File,
    time::Instant,
    io,
    io::{prelude::*, BufReader, Write},
    net::TcpStream
};

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //Reset terminal
    // let mut buffer = BufReader::new(stream.try_clone().unwrap());
    
    println!("Welcome to Hermod. The Client is starting so hang thight!");
    
    let mut stream = match TcpStream::connect("127.0.0.1:2088") {
        Ok(connection) => connection,
        Err(_) => {
            println!("Connection refused on port 2088");
            return
        }
    };

    //Sending deltoken to the DB host for authentication
    let tmptoken = "Ybd.";
    stream.write(format!("auth: {}\n", tmptoken).as_bytes());

    let mut read = [0; 128];
    stream.read(&mut read);
    println!("{}", str::from_utf8(&read).unwrap().trim_matches(char::from(0)));
    
    let stdin = io::stdin();
    
    loop {
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //Reset terminal
        // println!("So... What do you need?");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        stdin.read_line(&mut cmd);

        // println!("{:?}", cln);
        stream.write(format!("{}\n", &cmd[..cmd.len() - 1]).as_bytes());
        
        let mut response = String::new();
        let mut totallen = 0;
        loop {
            let mut read = [0; 128];
            let readlen = stream.read(&mut read).unwrap();
            
            let readutf8 = str::from_utf8(&read).unwrap().trim();
            response.push_str(readutf8);
            // println!("Read: {}", readutf8);
            if read[readlen - 1] == 4 {
                totallen += readlen;
                break;
            }
            totallen += 128;
        }
        println!("Response: {}\nQET: {}ns\nCODE: {}", &response[..totallen - 19], &response[totallen - 18..totallen - 6].trim(), &response[totallen - 5..totallen - 2]);
    }
}

fn test(mut stream: &TcpStream) {
    let mut output = File::create("./output.dat").unwrap();
    let mut test: (f64, usize) = (0.0, 0);

    let testlen = 1000000;
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
        let ts = Instant::now();
        // println!("Waiting on response...");
        let mut read = [0; 128];
        stream.read(&mut read);
        // println!("Read1: {:?}", read);
        // let mut read2 = Vec::new();
        // stream.read_to_end(&mut read2);
        // println!("Read2: {:?}", read2);
        // println!("Time: {:?}, Read: {:?}", ts.elapsed(), read);
        
        if i % 5 == 4 {
            // let res = str::from_utf8(&read).unwrap_or("-").trim_matches(char::from(0));
            // let res2 = str::from_utf8(&read2).unwrap_or("-").trim_matches(char::from(0));
            // println!("L: {}; {}", res, &res2);
            // if res2.len() >= 19 {
            //     test = (&res2[1..13].trim().parse::<f64>().unwrap_or(0.0) + test.0, test.1 + 1);
            //     let out = format!("{:8?}{}{}{}", i, "\t", &res2[1..13].trim(), "\n");
            //     output.write(out.as_bytes());
            // }

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
