// HermodDB Client
// Copyright(c) 2022-2023 Matteo Minardi <grokepeer@gmail.com>
// AGPL Licensed

use std::{
    str,
    // fs::File,
    // time::Instant,
    io,
    io::{prelude::*, Write, BufReader},
    net::TcpStream
};

fn main() {
    let apiv = "v0.4."; //API version
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
    let mut buffer = BufReader::new(stream.try_clone().unwrap());
    let mut bytes: Vec<u8> = Vec::new();    //Used to copy down form the TCP buffer

    let stdin = io::stdin();

    println!("What Token to use?");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut iotoken = String::new();
    stdin.read_line(&mut iotoken).unwrap();

    //If no token was typed then the Client will send the default token to the Host
    if iotoken.len() < 2 {
        iotoken = "tokenn".to_string(); //Double n at the end is necessary, do not remove
    }

    //Sending deltoken to the DB host for authentication
    match stream.write(format!("auth: {}\u{4}", &iotoken[..iotoken.len() - 1]).as_bytes()) {
        Err(_) => {
            println!("Couldn't send authentication to the Host. Make sure the Host is running and reachable.");
            return
        },
        _ => {}
    };

    //Reading connection confirmation
    buffer.read_until(0x4, &mut bytes).unwrap();
    let welmsg = str::from_utf8(&bytes).unwrap();

    let detailinit = welmsg.find("(").unwrap();
    if &welmsg[detailinit + 9..detailinit + 14] != apiv {
        println!("Host API version mismatch. Update the Client or the Host to match the major API version. (Client API {}x | Host API {})", apiv, &welmsg[detailinit + 9..detailinit + 15]);
        stream.write("ext".as_bytes()).unwrap();
        return
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //Reset terminal
    println!("{}", welmsg);
    
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        stdin.read_line(&mut cmd).unwrap();

        // Checks if the user called for exit or quit
        if &cmd[..cmd.len() - 1] == "exit" || &cmd[..cmd.len() - 1] == "quit" || &cmd[..cmd.len() - 1] == "ext" {
            stream.write("ext".as_bytes()).unwrap();
            println!("Connection to Host dropped");
            return
        }

        match stream.write(format!("{}\u{4}", &cmd[..cmd.len() - 1]).as_bytes()) {
            Err(_) => {
                println!("Couldn't send data to the Host. Make sure the Host is running and reachable.");
                return
            },
            _ => {}
        };
        
        let mut bytes: Vec<u8> = Vec::new();    //Used to copy down form the TCP buffer
        let totallen = buffer.read_until(0x4, &mut bytes).unwrap();    //Reading response from Host
        let response = str::from_utf8(&bytes).unwrap();

        println!("Response: {}\nQET: {}ns\nCODE: {}", &response[..totallen - 19], &response[totallen - 18..totallen - 6].trim(), &response[totallen - 5..totallen - 2]);
    }
}

/*fn test(mut stream: &TcpStream) {
    // let mut output = File::create("./output.dat").unwrap();
    let test: (f64, usize) = (0.0, 0);

    let testlen = 1000000;
    let persymbol = (testlen / 100, testlen / 100 - 1);

    let timestart = Instant::now();

    print!("<");
    io::stdout().flush().unwrap();
    for i in 1..testlen {
        let cmd = format!("set {i} in _basedb to testing\n");
        stream.write(cmd.as_bytes()).unwrap();
        let mut read = [0; 128];
        stream.read(&mut read).unwrap();
        
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
        stream.write(cmd.as_bytes()).unwrap();
        // let ts = Instant::now();
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
}*/
