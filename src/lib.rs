use std::error::Error;

pub enum NetProtocol {
    TCP,
    UDP,
}

pub struct Config {
    pub lhost: String,
    pub lport: u16,
    pub rhost: String,
    pub rport: u16,
    pub proto: NetProtocol,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let lhost:String;
        let lport:u16;
        let rhost:String;
        let rport:u16;
        if args.len() < 2 {
            return Err("not enough arguments")
        }
        // -l : listen mode
        if args [1] == "-l" {
            println!("listen mode");
            lhost = String::from("0.0.0.0");
            lport = args[2].parse::<u16>().unwrap();
            rhost = String::from("0.0.0.0");
            rport = 0;
        }
        else {
            println!("connect mode");
            lhost = String::from("0.0.0.0");
            lport = 0;
            rhost = args[1].clone();
            rport = args[2].parse::<u16>().unwrap();
        }
        Ok(Config{lhost: lhost, lport: lport, rhost: rhost, rport: rport, proto: NetProtocol::TCP})
    }
}

// Create config

// Listen TCP

// Connect TCP

// Listen UDP

// Connect UDP

// run 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("run..");
    println!("{}", config.rhost);
    println!("{}", config.rport);
    println!("{}", config.lhost);
    println!("{}", config.lport);
    Ok(())
}
