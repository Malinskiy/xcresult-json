extern crate plist;

mod xcresult;
mod cas;
mod test_result;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde_json::Value;
use crate::{test_result::TestResult, cas::ContentAddressableStorage};

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

    match &cli.command {
        Some(Commands::Convert { output }) => {
            let test_result = TestResult::new(&input.to_string_lossy());
            let xcresult = test_result.read().expect("invalid Info.plist");
            let root_id = xcresult.root_id;
            let root_obj = test_result.retrieve(&root_id.hash, &cas::ObjectType::JSON);
            let json = root_obj.unwrap();
            let mut v: Value = serde_json::from_str(&json).unwrap();
            let converted = test_result.convert(&mut v);
            println!("{}", converted.expect("convertion failed"));
        }
        None => {}
    }
}

