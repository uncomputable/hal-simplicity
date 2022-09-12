use simplicity::core::iter::DagIterable;
use simplicity::core::redeem::RefWrapper;
use simplicity::core::{RedeemNode, Value};
use simplicity::jet::Application;
use std::collections::{HashMap, HashSet};

/// Print the given program in a Graphviz-parseable format.
pub fn print_program<App: Application>(
    program: &RedeemNode<App>,
    node_to_scribe: &HashMap<RefWrapper<App>, Value>,
) {
    println!("digraph {{\nranksep=3;");

    let connected = compute_connected_component(RefWrapper(program), node_to_scribe);
    let mut node_to_index = HashMap::new();

    for (index, node) in RefWrapper(program).iter_post_order().enumerate() {
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
fn compute_connected_component<'a, App: Application>(
    program: RefWrapper<'a, App>,
    node_to_scribe: &HashMap<RefWrapper<App>, Value>,
) -> HashSet<RefWrapper<'a, App>> {
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
    if let Ok(bytes) = value.try_to_bytes() {
        print!("{} [label=\"scribe\\n0x", index);

        for byte in &bytes {
            print!("{:02X}", byte);
        }

        println!("\\n1 → 2^{}\"]", bytes.len() * 8);
    } else {
        unimplemented!("Can only display scribe values that are bytes")
    }
}

/// Print the given node in a Graphviz-parsable format.
fn print_node<App: Application>(
    node: RefWrapper<App>,
    index: usize,
    node_to_index: &HashMap<RefWrapper<App>, usize>,
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
