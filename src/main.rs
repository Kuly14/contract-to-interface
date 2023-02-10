use interface::Contract;
use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to your solidity contract file
    pub path: PathBuf,
    /// Use -p if you want to print the interface to the terminal
    #[clap(long, short, action)]
    pub print: bool,
    /// Use -d to specify where do you want to save the interface. If the file exists it will error
    #[clap(long, short)]
    pub destination: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let contract = Contract::build(&args.path)?;
    let interface = contract.format();

    if args.print {
        println!("{}", interface);
    }

    if let Some(dest) = args.destination {
        let mut file = File::create(&dest)?;
        file.write_all(interface.as_bytes())?;

        println!("Done...");
        println!("Bytecode is saved at {:#?}", dest);
    } else {
        let mut new_name = String::from("I");
        new_name.push_str(&contract.name);
        new_name.push_str(&".sol");
        let file_name = args.path.file_name().unwrap().to_str().unwrap();
        let path = args.path.to_str().unwrap().replace(file_name, &new_name);
        let mut file = File::create(path)?;
        file.write_all(interface.as_bytes())?;
    }

    Ok(())
}
