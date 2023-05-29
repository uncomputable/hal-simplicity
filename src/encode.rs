use base64::engine::general_purpose;
use base64::Engine;
use simplicity::bitwriter::BitWriter;
use simplicity::jet::Jet;
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

    general_purpose::STANDARD.encode(&program_bytes)
}

/// Encode the given program commitment as base64 string.
pub fn encode_program_dummy_witness<J: Jet>(program: &CommitNode<J>) -> String {
    encode_base64(|w| program.encode(w))
}

/// Encode the given program with witness data as base64 string.
#[allow(dead_code)]
pub fn encode_program<J: Jet>(program: &RedeemNode<J>) -> String {
    encode_base64(|w| program.encode(w))
}
