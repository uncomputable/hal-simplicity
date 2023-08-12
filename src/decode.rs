use std::sync::Arc;

use base64::engine::general_purpose;
use base64::Engine;
use simplicity::jet::Jet;
use simplicity::{BitIter, RedeemNode};

use crate::error::Error;

/// Return a bit iterator over the given base64 string.
fn get_bit_iter(base64_string: &str) -> Result<BitIter<impl Iterator<Item = u8>>, Error> {
    let program_bytes = general_purpose::STANDARD.decode(base64_string)?;
    Ok(BitIter::new(program_bytes.into_iter()))
}

/// Decode a program with witness data from the given base64 string.
pub fn decode_program<J: Jet>(base64: &str) -> Result<Arc<RedeemNode<J>>, Error> {
    let mut bits = get_bit_iter(base64)?;
    let program = RedeemNode::decode(&mut bits)?;
    Ok(program)
}
