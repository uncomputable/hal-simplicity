mod compress;
mod decode;
mod encode;
mod error;
mod graph;
mod tx;

use crate::error::Error;
use crate::tx::TransactionInfo;
use clap::{Parser, Subcommand};
use elements::hashes::hex::FromHex;
use hal_elements::GetInfo;
use simplicity::jet::Elements;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List nodes of Simplicity program
    List {
        /// Base 64 encoding of Simplicity program
        base64: String,
    },
    /// Visualize Simplicity program as graph
    ///
    /// Output is saved to `simplicity.svg`
    Graph {
        /// Base 64 encoding of Simplicity program
        base64: String,
    },
    Tx {
        #[command(subcommand)]
        command: TxCommand,
    },
}

#[derive(Subcommand)]
enum TxCommand {
    /// Decode a raw transaction to JSON
    Decode {
        /// Raw transaction hex
        hex: String,
    },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::List { base64 } => {
            let program = decode::decode_program_dummy_witness::<Elements>(&base64);
            println!("{}", program);
        }
        Command::Graph { base64 } => {
            let program = decode::decode_program_dummy_witness::<Elements>(&base64);
            let node_to_scribe = compress::compress_scribe(&program);
            graph::visualize(&program, &node_to_scribe)?;
        }
        Command::Tx { command } => match command {
            TxCommand::Decode { hex } => {
                let tx_bytes = Vec::<u8>::from_hex(hex.as_str())?;
                let tx: elements::Transaction = elements::encode::deserialize(&tx_bytes)?;
                let info: TransactionInfo = tx.get_info(hal_elements::Network::ElementsRegtest);
                serde_json::to_writer_pretty(std::io::stdout(), &info)?;
            }
        },
    }

    Ok(())
}
