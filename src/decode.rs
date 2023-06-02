use crate::error::Error;
use base64::engine::general_purpose;
use base64::Engine;
use simplicity::bititer::BitIter;
use simplicity::core::Value;
use simplicity::jet::Jet;
use simplicity::{CommitNode, RedeemNode};
use std::rc::Rc;

/// Return a bit iterator over the given base64 string.
fn get_bit_iter(base64_string: &str) -> Result<BitIter<impl Iterator<Item = u8>>, Error> {
    let program_bytes = general_purpose::STANDARD.decode(base64_string)?;
    Ok(BitIter::new(program_bytes.into_iter()))
}

/// Decode a program commitment from the given base64 string.
pub fn decode_program_no_witness<J: Jet>(base64: &str) -> Result<Rc<CommitNode<J>>, Error> {
    let mut bits = get_bit_iter(base64)?;
    let commit = CommitNode::decode(&mut bits)?;
    Ok(commit)
}

/// Decode a program with dummy witness data from the given base64 string.
/// The witness is statically set to a sequence of `Value::Unit`.
pub fn decode_program_dummy_witness<J: Jet>(base64: &str) -> Result<Rc<RedeemNode<J>>, Error> {
    let commit = decode_program_no_witness(base64)?;
    let program = commit.finalize(std::iter::repeat(Value::Unit))?;
    Ok(program)
}

/// Decode a program with witness data from the given base64 string.
#[allow(dead_code)]
pub fn decode_program<J: Jet>(base64: &str) -> Result<Rc<RedeemNode<J>>, Error> {
    let mut bits = get_bit_iter(base64)?;
    let program = RedeemNode::decode(&mut bits)?;
    Ok(program)
}
