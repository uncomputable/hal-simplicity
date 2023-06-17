use crate::error::Error;
use base64::engine::general_purpose;
use base64::Engine;
use simplicity::BitWriter;
use std::io;

/// Encode a program as base64 string, using the given function over a bit writer.
pub fn encode_base64<F>(f: F) -> Result<String, Error>
where
    F: Fn(&mut BitWriter<&mut Vec<u8>>) -> io::Result<usize>,
{
    let mut program_bytes = Vec::new();
    let mut w = BitWriter::new(&mut program_bytes);
    f(&mut w).map(|_| w.flush_all())??;
    Ok(general_purpose::STANDARD.encode(&program_bytes))
}
