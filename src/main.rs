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

// TODO: we want per version metadata, this means that dependencies, conflicts, and so on are
// specified inside the build script. This also means that we will no longer have a versions/
// directory, instead the different versions will just be inside the root of the ports directory

fn main() {
    let args = Args::parse();

    if let Err(err) = ports::handle(args.action, args.specifiers, args.rebuild) {
        eprintln!("error: {}", err);
    }
}


