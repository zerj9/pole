// #![allow(unused)]
use bollard::Docker;
use clap::{Parser, Subcommand};
use pole::{deploy, verify};
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Verify {
        #[clap(short, value_parser, required(true))]
        config: Value,
    },
    Deploy {
        #[clap(short, required(true))]
        config: Value,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let docker = Docker::connect_with_local_defaults().unwrap();

    println!("Docker version: {:?}", docker.version().await.unwrap());

    match args.command {
        Subcommands::Verify { config } => verify(config),
        Subcommands::Deploy { config } => deploy(config),
    }
}
