extern crate plist;

mod cas;
mod convert_interactor;
mod test_result;
mod xcresult;

use clap::{Parser, Subcommand};
use convert_interactor::ConvertInteractor;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let input = cli.input;

    simple_logger::SimpleLogger::new().env().init().unwrap();

    match &cli.command {
        Some(Commands::Convert { output }) => {
            let interactor = ConvertInteractor::new();
            interactor
                .execute(&input, output)
                .expect("conversion failed")
        }
        None => {}
    }
}
