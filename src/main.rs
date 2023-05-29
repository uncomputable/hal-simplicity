mod compress;
mod decode;
mod encode;
mod graphviz;
mod policy;

use clap::{App, Arg, Command};
use simplicity::jet::Elements;

fn main() {
    let list_subcommand = Command::new("list")
        .about("List the nodes of a Simplicity program")
        .arg_required_else_help(true)
        .arg(Arg::new("base64_string").help("base64 encoding of Simplicity program"));
    let graph_subcommand = Command::new("graph")
        .about("Visualize a Simplicity program as a graph")
        .arg_required_else_help(true)
        .arg(Arg::new("base64_string").help("base64 encoding of Simplicity program"))
        .after_help(
            "Compile to PNG via `simple-companion [base64_string] | dot -Tpng -o file.png`",
        );
    let script_subcommand = Command::new("script")
        .about("Parse the given hex as Miniscript, convert into equivalent Simplicity and export the resulting program")
        .arg_required_else_help(true)
        .arg(Arg::new("hex_string").help(" hex encoding of Miniscript (Tapscript only)").required(true))
        .after_help("EXAMPLES:\n\n\
        Single key:\n20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac\n\
        One of two keys (equally likely):\n20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac7c20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac9b\n\
        A user and a 2FA service need to sign off, but after 90 days the user alone is enough:\n20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ad20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac736402a032b268\n\
        A 3-of-3 that turns into a 2-of-3 after 90 days:\n\
        20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac7c20b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac937c20387671353e273264c495656e27e39ba899ea8fee3bb69fb2a680e22093447d48ac937c63006702a032b29268935387\n\
        The BOLT #3 to_local policy:\n\
        20d85a959b0290bf19bb89ed43c916be835475d013da4b362117393e25a48229b8ac6420b617298552a72ade070667e86ca63b8f5789a9fe8731ef91202a91c9f3459007ac6702f003b268");

    let app = App::new("Simple Companion")
        .about("Companion for the Simplicity language")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Christian Lewe <clewe@blockstream.com>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(list_subcommand)
        .subcommand(graph_subcommand)
        .subcommand(script_subcommand);

    let matches = app.get_matches();

    match matches.subcommand().expect("subcommand_required") {
        ("list", arg_matches) => {
            let base64_string = arg_matches
                .get_one::<String>("base64_string")
                .expect("positional");

            let program = decode::decode_program_dummy_witness::<Elements>(base64_string);
            println!("{}", program);
        }
        ("graph", arg_matches) => {
            let base64_string = arg_matches
                .get_one::<String>("base64_string")
                .expect("positional");

            let program = decode::decode_program_dummy_witness::<Elements>(base64_string);
            let node_to_scribe = compress::compress_scribe(&program);
            graphviz::print_program(&program, &node_to_scribe);
        }
        ("script", arg_matches) => {
            let hex_string = arg_matches
                .get_one::<String>("hex_string")
                .expect("positional");

            let policy = policy::parse_miniscript(hex_string);
            let program = policy::compile(&policy);
            println!(
                "Simplicity program without witness:\n{}",
                encode::encode_program_dummy_witness(&program)
            );
        }
        _ => {
            unreachable!("arg_required_else_help")
        }
    }
}
