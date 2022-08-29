use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct KvsArg {
    #[clap(subcommand)]
    cmd: KvsCommand,
}

#[derive(Subcommand)]
enum KvsCommand {
    /// Insert the key-value pair
    Set { key: String, value: String },
    /// Fetch the value of specified key
    Get { key: String },
    /// Remove key-value pair
    Rm { key: String },
}

fn main() {
    let cli = KvsArg::parse();
    match cli.cmd {
        #[allow(unused_variables)]
        KvsCommand::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        #[allow(unused_variables)]
        KvsCommand::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        #[allow(unused_variables)]
        KvsCommand::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
