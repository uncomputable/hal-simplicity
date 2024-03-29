mod compress;
mod decode;
mod encode;
mod error;
mod graph;
mod tx;
mod util;

use clap::{Parser, Subcommand};
use elements::hex::FromHex;
use simplicity::elements;
use simplicity::jet::Elements;

use crate::error::Error;
use crate::tx::TransactionInfo;
use crate::util::{GetInfo, Network};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Work with Simplicity programs
    Prog {
        #[command(subcommand)]
        command: ProgCommand,
    },
    /// Work with Elements transactions
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

#[derive(Subcommand)]
enum ProgCommand {
    /// List nodes of program
    List {
        /// Base 64 encoding of program
        base64: String,
    },
    /// Visualize program as graph
    ///
    /// Output is saved to `simplicity.svg`
    Graph {
        /// Base 64 encoding of program
        base64: String,
    },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Prog {
            command: ProgCommand::List { base64 },
        } => {
            let program = decode::decode_program::<Elements>(&base64)?;
            println!("{}", program);
        }
        Command::Prog {
            command: ProgCommand::Graph { base64 },
        } => {
            let program = decode::decode_program::<Elements>(&base64)?;
            graph::visualize(program.as_ref())?;
        }
        Command::Tx { command } => match command {
            TxCommand::Decode { hex } => {
                let tx_bytes = Vec::<u8>::from_hex(hex.as_str()).expect("hex error");
                let tx: elements::Transaction = elements::encode::deserialize(&tx_bytes)?;
                let info: TransactionInfo = tx.get_info(Network::ElementsRegtest);
                serde_json::to_writer_pretty(std::io::stdout(), &info)?;
            }
        },
    }

    Ok(())
}
