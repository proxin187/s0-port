mod error;
mod ports;

use clap::{Parser, ValueEnum};


#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
    /// Install one or more ports
    Install,
    /// Remove one or more ports from the system
    Remove,
}

#[derive(Parser)]
pub struct Args {
    /// Action to perform on specifiers
    action: Action,

    /// Specifier of one or more ports
    specifiers: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = ports::handle(args.action, args.specifiers) {
        eprintln!("error: {}", err);
    }
}


