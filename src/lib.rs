use std::{error::Error, io::Write, net::{TcpListener, TcpStream}, thread};

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

// Listen TCP
pub fn listen_tcp(host: String, port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind((host, port)).unwrap();

    //accepts connections from clients
    for stream in listener.incoming() {
        println!("New connection incomming!");
        thread::spawn(move || {
            match stream {
                Ok(mut stream) => {
                    loop {
                        stream.write(b"Hello\n");
                    }
                    println!("end");
                },
                Err(e) => panic!("Error: {e}"),
            }
        });
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
