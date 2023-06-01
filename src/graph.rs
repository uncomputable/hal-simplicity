use crate::error::Error;
use layout::backends::svg::SVGWriter;
use layout::gv::{DotParser, GraphBuilder};
use simplicity::core::iter::DagIterable;
use simplicity::core::{redeem, RedeemNode, Value};
use simplicity::jet::Jet;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IOWrite;
use std::path::Path;

pub fn visualize<J: Jet>(
    program: &RedeemNode<J>,
    node_to_scribe: &HashMap<redeem::RefWrapper<J>, Value>,
) -> Result<(), Error> {
    let dot = program_to_dot(program, node_to_scribe)?;
    dot_to_svg(&dot, "simplicity.svg")
}

fn dot_to_svg<P: AsRef<Path>>(dot: &str, path: P) -> Result<(), Error> {
    let mut parser = DotParser::new(dot);
    let graph = parser.process().expect("invalid dot string");

    let mut gb = GraphBuilder::new();
    gb.visit_graph(&graph);
    let mut vg = gb.get();

    let mut writer = SVGWriter::new();
    vg.do_it(false, false, false, &mut writer);
    let svg = writer.finalize();
    let file = File::create(path)?;
    write!(&file, "{}", svg)?;

    Ok(())
}

fn program_to_dot<J: Jet>(
    program: &RedeemNode<J>,
    node_to_scribe: &HashMap<redeem::RefWrapper<J>, Value>,
) -> Result<String, Error> {
    let mut dot = String::new();
    writeln!(dot, "digraph {{\nranksep=3;")?;

    let reachable = reachable_nodes(redeem::RefWrapper(program), node_to_scribe);
    let mut node_to_index = HashMap::new();

    for (index, node) in redeem::RefWrapper(program).iter_post_order().enumerate() {
        if !reachable.contains(&node) {
            continue;
        }

        if let Some(value) = node_to_scribe.get(&node) {
            fmt_scribe(&mut dot, value, index)?;
        } else {
            fmt_node(&mut dot, node, index, &node_to_index)?;
        }

        node_to_index.insert(node, index);
    }

    writeln!(&mut dot, "}}")?;
    Ok(dot)
}

/// Compute nodes that are reachable from root without entering scribe expressions
///
/// Nodes inside scribe expressions may be shared and thus reachable.
/// Therefore, the opposite approach of computing reachable nodes from scribe roots does not work.
fn reachable_nodes<'a, J: Jet>(
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

fn fmt_scribe<W: FmtWrite>(w: &mut W, value: &Value, index: usize) -> fmt::Result {
    let (bytes, bit_len) = value.to_bytes_len();

    write!(w, "{} [label=\"", index)?;

    // scribe(·) = unit
    if bit_len == 0 {
        write!(w, "unit")?;
    }

    if bit_len % 8 == 0 {
        for byte in &bytes {
            write!(w, "{:02X}", byte)?;
        }
    } else {
        for byte in &bytes {
            write!(w, "{:08b}", byte)?;
        }
    }

    match bit_len {
        0 => writeln!(w, "\\n1 → 1\"]"),
        1 => writeln!(w, "\\n1 → 2\"]"),
        n => writeln!(w, "\\n1 → 2^{}\"]", n),
    }
}

fn fmt_node<J: Jet, W: FmtWrite>(
    w: &mut W,
    node: redeem::RefWrapper<J>,
    index: usize,
    node_to_index: &HashMap<redeem::RefWrapper<J>, usize>,
) -> fmt::Result {
    write!(w, "{} [label=\"{}\\n{}\"];", index, node.0.inner, node.0.ty)?;

    if let Some(left) = node.get_left() {
        let i_abs = node_to_index.get(&left).expect("post order");

        if let Some(right) = node.get_right() {
            let j_abs = node_to_index.get(&right).expect("post order");
            writeln!(w, "  {} -> {} [color=red];", index, i_abs)?;
            writeln!(w, "  {} -> {} [color=blue];", index, j_abs)?;
        } else {
            writeln!(w, "  {} -> {};", index, i_abs)?;
        }
    }

    Ok(())
}
