mod error;
mod ports;

use clap::{Parser, ValueEnum};


#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
    /// Install one or more ports
    Install,
    /// Remove one or more ports
    Remove,
}

#[derive(Parser)]
pub struct Args {
    /// The action you want
    action: Action,

    /// One or more ports
    specifiers: Vec<String>,

    /// Rebuild ports, including those already installed
    #[clap(long, short, action)]
    rebuild: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = ports::handle(args.action, args.specifiers, args.rebuild) {
        eprintln!("error: {}", err);
    }
}


