use base64::engine::general_purpose;
use base64::Engine;
use simplicity::bitwriter::BitWriter;
use std::{io, process};

/// Encode a program as base64 string, using the given function over a bit writer.
pub fn encode_base64<F>(f: F) -> String
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
