use std::{error::Error, io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};

#[derive (Debug)]
pub enum NetProtocol {
    TCP,
    UDP,
}

#[derive (Debug)]
pub enum NetMode {
    Connect,
    Listen,
}

#[derive (Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub proto: NetProtocol,
    pub mode: NetMode,
}

impl Config {
    // Create config
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let host:String;
        let port:u16;
        let mode:NetMode;
        if args.len() < 2 {
            return Err("not enough arguments")
        }
        // -l : listen mode
        if args [1] == "-l" {
            println!("listen mode");
            host = String::from("0.0.0.0");
            port = args[2].parse::<u16>().unwrap();
            mode = NetMode::Listen
        }
        else {
            println!("connect mode");
            host = args[1].clone();
            port = args[2].parse::<u16>().unwrap();
            mode = NetMode::Connect
        }
        Ok(Config{host, port, proto: NetProtocol::TCP, mode})
    }
}

fn recv_bytes(mut stream: TcpStream) {
    let mut buffer = [0;256];
    loop {
        match stream.read(&mut buffer[..]) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("end");
                }
                std::io::stdout().write_all(&buffer).unwrap();
            }
            Err(e) => panic!("Error: {e}"),
        }
    }
}

fn send_bytes(mut stream: TcpStream) {
    let mut buffer = [0;256];
    loop {
        match io::stdin().read(&mut buffer[..]) {
            Ok(bytes_read) => {
                stream.write(&mut buffer).unwrap();
                println!("{} bytes sent to client.", bytes_read)
            }
            Err(e) => panic!("Error: {e}"),
        }
    }
}

// Handle client
fn handle_client(stream: TcpStream) {
    let stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || send_bytes(stream));
    thread::spawn(move || recv_bytes(stream_clone));
}

// Listen TCP
pub fn listen_tcp(host: String, port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind((host, port)).unwrap();

    //accepts connections from clients
    for stream in listener.incoming() {
        println!("New connection incomming!");
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream)).join().unwrap()
            },
            Err(e) => panic!("Error: {e}"),
        }
    }
    Ok(())
}

// Connect TCP

// Listen UDP

// Connect UDP

// run 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("run..");
    println!("{:?}", config);
    match config.proto {
        NetProtocol::TCP => match config.mode {
            NetMode::Listen => listen_tcp(config.host, config.port)?,
            NetMode::Connect => println!("Not supported... yet!")
        },
        NetProtocol::UDP => println!("Not supported... yet!")
    }
    Ok(())
}
