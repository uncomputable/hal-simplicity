use simplicity::core::iter::DagIterable;
use simplicity::core::{redeem, RedeemNode, Value};
use simplicity::jet::Jet;
use std::collections::{HashMap, HashSet};

/// Print the given program in a Graphviz-parseable format.
pub fn print_program<J: Jet>(
    program: &RedeemNode<J>,
    node_to_scribe: &HashMap<redeem::RefWrapper<J>, Value>,
) {
    println!("digraph {{\nranksep=3;");

    let connected = compute_connected_component(redeem::RefWrapper(program), node_to_scribe);
    let mut node_to_index = HashMap::new();

    for (index, node) in redeem::RefWrapper(program).iter_post_order().enumerate() {
        if !connected.contains(&node) {
            continue;
        }

        if let Some(value) = node_to_scribe.get(&node) {
            print_scribe(value, index);
        } else {
            print_node(node, index, &node_to_index);
        }

        node_to_index.insert(node, index);
    }

    println!("}}");
}

/// Compute the connected component of the given program by traversing from the root in pre-order.
/// Scribe expressions are treated as leaves.
fn compute_connected_component<'a, J: Jet>(
    program: redeem::RefWrapper<'a, J>,
    node_to_scribe: &HashMap<redeem::RefWrapper<J>, Value>,
) -> HashSet<redeem::RefWrapper<'a, J>> {
    let mut visited = HashSet::new();
    let mut stack = vec![program];

    while let Some(top) = stack.pop() {
        visited.insert(top);

        if let Some(left) = top.get_left() {
            if !node_to_scribe.contains_key(&top) && !visited.contains(&left) {
                stack.push(left);
            }
        }
        if let Some(right) = top.get_right() {
            if !node_to_scribe.contains_key(&top) && !visited.contains(&right) {
                stack.push(right);
            }
        }
    }

    visited
}

/// Print the given scribe expression in a Graphviz-parsable format.
fn print_scribe(value: &Value, index: usize) {
    let (bytes, bit_len) = value.to_bytes_len();
    print!("{} [label=\"scribe\\n", index);

    if bit_len % 8 == 0 {
        for byte in &bytes {
            print!("{:02X}", byte);
        }
    } else {
        for byte in &bytes {
            print!("{:08b}", byte)
        }
    }

    println!("\\n1 → 2^{}\"]", bit_len);
}

/// Print the given node in a Graphviz-parsable format.
fn print_node<J: Jet>(
    node: redeem::RefWrapper<J>,
    index: usize,
    node_to_index: &HashMap<redeem::RefWrapper<J>, usize>,
) {
    print!("{} [label=\"{}\\n{}\"];", index, node.0.inner, node.0.ty);

    if let Some(left) = node.get_left() {
        let i_abs = node_to_index.get(&left).unwrap();

        if let Some(right) = node.get_right() {
            let j_abs = node_to_index.get(&right).unwrap();
            println!("  {} -> {} [color=red];", index, i_abs);
            println!("  {} -> {} [color=blue];", index, j_abs);
        } else {
            println!("  {} -> {};", index, i_abs);
        }
    }
}
