mod error;
mod port;

use clap::{Parser, ValueEnum};


// doas portd pull && doas portd fetch sbase ubase
// doas portd clean neofetch


#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
    /// Fetch, build and install one or more ports
    Fetch,
    /// Clean one or more ports from the system
    Clean,
}

impl Action {
    pub fn args(&self) -> &'static str {
        match self {
            Action::Fetch => "fetch",
            Action::Clean => "clean",
        }
    }
}

#[derive(Parser)]
pub struct Args {
    action: Action,
    ports: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = port::ports(args.ports, args.action) {
        eprintln!("error: {}", err);
    }
}


