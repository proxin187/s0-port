mod error;
mod port;

use port::Port;

use clap::{Parser, Subcommand};


// doas portd pull && doas portd fetch sbase ubase
// doas portd clean neofetch

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Pull the latest ports repo (remote through $PORTD_REMOTE)
    Pull {
        #[arg(long, short)]
        dry_run: bool,
    },
    /// Fetch, build and install one or more ports
    Fetch {
        port: String,
    },
    /// Clean one or more ports from the system
    Clean {
        port: String,
    },
}

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Command,

    /// Suppress non-error output
    #[arg(long, short)]
    silent: bool,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Pull { dry_run } => {
        },
        Command::Fetch { port } => {
            let Ok(port) = Port::find(port).inspect_err(|err| eprintln!("error: {}", err)) else { return };

            if let Err(err) = port.fetch() {
                eprintln!("error: {}", err);
            }
        },
        Command::Clean { port } => {
        },
    }
}


