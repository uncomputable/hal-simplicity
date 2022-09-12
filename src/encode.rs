use simplicity::bitwriter::BitWriter;
use simplicity::jet::Application;
use simplicity::{CommitNode, RedeemNode};
use std::{io, process};

/// Encode a program as base64 string, using the given function over a bit writer.
fn encode_base64<F>(f: F) -> String
where
    F: Fn(&mut BitWriter<&mut Vec<u8>>) -> io::Result<usize>,
{
    let mut program_bytes = Vec::new();
    let mut w = BitWriter::new(&mut program_bytes);

    match f(&mut w).map(|_| w.flush_all()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to encode Simplicity program to bits: {}", e);
            process::exit(1);
        }
    }

    base64::encode(&program_bytes)
}

/// Encode the given program commitment as base64 string.
pub fn encode_program_dummy_witness<App: Application>(program: &CommitNode<App>) -> String {
    encode_base64(|w| program.encode(w))
}

/// Encode the given program with witness data as base64 string.
#[allow(dead_code)]
pub fn encode_program<App: Application>(program: &RedeemNode<App>) -> String {
    encode_base64(|w| program.encode(w))
}
