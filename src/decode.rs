use base64::engine::general_purpose;
use base64::Engine;
use simplicity::bititer::BitIter;
use simplicity::core::Value;
use simplicity::jet::Jet;
use simplicity::{CommitNode, RedeemNode};
use std::process;
use std::rc::Rc;

/// Return a bit iterator over the given base64 string.
fn get_bit_iter(base64_string: &str) -> BitIter<impl Iterator<Item = u8>> {
    let program_bytes = match general_purpose::STANDARD.decode(base64_string) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to decode base64 string: {}", e);
            process::exit(1);
        }
    };

    BitIter::new(program_bytes.into_iter())
}

/// Decode a program commitment from the given base64 string.
pub fn decode_program_no_witness<J: Jet>(base64: &str) -> Rc<CommitNode<J>> {
    let mut bits = get_bit_iter(base64);

    match CommitNode::decode(&mut bits) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to decode Simplicity program from bits: {}", e);
            process::exit(1);
        }
    }
}

/// Decode a program with dummy witness data from the given base64 string.
/// The witness is statically set to a sequence of `Value::Unit`.
pub fn decode_program_dummy_witness<J: Jet>(base64: &str) -> Rc<RedeemNode<J>> {
    let commit = decode_program_no_witness(base64);

    match commit.finalize(std::iter::repeat(Value::Unit)) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to type-check Simplicity program: {}", e);
            process::exit(1);
        }
    }
}

/// Decode a program with witness data from the given base64 string.
#[allow(dead_code)]
pub fn decode_program<J: Jet>(base64: &str) -> Rc<RedeemNode<J>> {
    let mut bits = get_bit_iter(base64);

    match RedeemNode::decode(&mut bits) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to decode Simplicity program from bits: {}", e);
            process::exit(1);
        }
    }
}
