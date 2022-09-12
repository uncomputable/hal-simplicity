use simplicity::bititer::BitIter;
use simplicity::core::Value;
use simplicity::jet::Application;
use simplicity::{CommitNode, RedeemNode};
use std::process;
use std::rc::Rc;

/// Return a bit iterator over the given base64 string.
fn get_bit_iter(base64_string: &str) -> BitIter<impl Iterator<Item = u8>> {
    let program_bytes = match base64::decode(base64_string) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to decode base64 string: {}", e);
            process::exit(1);
        }
    };

    BitIter::new(program_bytes.into_iter())
}

/// Decode a program commitment from the given base64 string.
pub fn decode_program_no_witness<App: Application>(base64_string: &str) -> Rc<CommitNode<App>> {
    let mut bits = get_bit_iter(base64_string);

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
pub fn decode_program_dummy_witness<App: Application>(base64_string: &str) -> Rc<RedeemNode<App>> {
    let commit = decode_program_no_witness(base64_string);

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
pub fn decode_program<App: Application>(base64_string: &str) -> Rc<RedeemNode<App>> {
    let mut bits = get_bit_iter(base64_string);

    match RedeemNode::decode(&mut bits) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to decode Simplicity program from bits: {}", e);
            process::exit(1);
        }
    }
}
