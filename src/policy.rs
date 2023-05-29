use simplicity::bitcoin_hashes::hex::FromHex;
use simplicity::core::Context;
use simplicity::jet::Elements;
use simplicity::miniscript::{Miniscript, MiniscriptKey, Tap};
use simplicity::policy::ast::Policy;
use simplicity::policy::key::PublicKey32;
use simplicity::{bitcoin, elements, CommitNode};
use std::process;
use std::rc::Rc;

/// Parse the given hex string as Miniscript (Tapscript)
/// and convert into an equivalent Simplicity policy.
pub fn parse_miniscript(hex_string: &str) -> Policy<bitcoin::XOnlyPublicKey> {
    let u8_vector = match Vec::<u8>::from_hex(hex_string) {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "Hex encoding of Bitcoin Script must be of even length: {}",
                e
            );
            process::exit(1);
        }
    };

    let script = elements::Script::from(u8_vector);
    println!("Bitcoin Script:\n{}\n", script);

    let miniscript = match Miniscript::<_, Tap>::parse(&script) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to parse Bitcoin Script as Miniscript: {}", e);
            process::exit(1);
        }
    };
    println!("Miniscript:\n{}\n", miniscript);

    match Policy::try_from(&miniscript) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to parse Miniscript as Simplicity policy: {}", e);
            process::exit(1);
        }
    }
}

/// Compile the given Simplicity policy into an equivalent program commitment.
pub fn compile<Pk: MiniscriptKey + PublicKey32>(policy: &Policy<Pk>) -> Rc<CommitNode<Elements>> {
    let mut context = Context::default();
    match policy.compile(&mut context) {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "Failed to compile Simplicity policy to Simplicity program: {}",
                e
            );
            process::exit(1);
        }
    }
}
