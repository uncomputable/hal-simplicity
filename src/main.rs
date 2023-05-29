mod compress;
mod decode;
mod encode;
mod graphviz;
mod policy;

use clap::{Parser, Subcommand};
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
    /// Compile to PNG via `simple-companion [base64_string] | dot -Tpng -o file.png`
    Graph {
        /// Base 64 encoding of Simplicity program
        base64: String,
    },
    /// Convert Miniscript into equivalent Simplicity program and export in base64
    Script {
        /// Hex encoding of Miniscript
        ///
        /// Single key: 20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac
        ///
        /// 1-of-2: 20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac7c20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac9b
        ///
        /// User+2FA, or user after 90 days: 20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ad20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac736402a032b268
        ///
        /// 3-of-3, or 2-of-3 after 90 days: 20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac7c20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac937c20387671353e273264c495656e27e39ba899ea8fee3bb69fb2a680e22093447d48ac937c63006702a032b29268935387
        ///
        /// Bolt 3 to_local: 20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac6420b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac6702f003b268
        hex: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List { base64 } => {
            let program = decode::decode_program_dummy_witness::<Elements>(&base64);
            println!("{}", program);
        }
        Command::Graph { base64 } => {
            let program = decode::decode_program_dummy_witness::<Elements>(&base64);
            let node_to_scribe = compress::compress_scribe(&program);
            graphviz::print_program(&program, &node_to_scribe);
        }
        Command::Script { hex } => {
            let policy = policy::parse_miniscript(&hex);
            let program = policy::compile(&policy);
            println!(
                "Simplicity program without witness:\n{}",
                encode::encode_program_dummy_witness(&program)
            );
        }
    }
}
