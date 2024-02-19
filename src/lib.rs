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
        // -l : listen mode
        if args.len() < 2 {
            return Err("not enough arguments")
        }
        if args [1] == "-l" {
            println!("listen mode");
        }
        else {
            println!("connect mode");
        }
        Ok(Config{lhost: "LHOST".to_string(), lport: 01, rhost: "RHOST".to_string(), rport: 02, proto: NetProtocol::TCP})
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
    Ok(())
}
