use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IOWrite;
use std::path::Path;

use layout::backends::svg::SVGWriter;
use layout::gv::{DotParser, GraphBuilder};
use simplicity::dag::{DagLike, MaxSharing, PostOrderIterItem};
use simplicity::jet::Jet;
use simplicity::Value;
use simplicity::{Imr, RedeemNode};

use crate::compress;
use crate::error::Error;

pub fn visualize<J: Jet>(program: &RedeemNode<J>) -> Result<(), Error> {
    let dot = program_to_dot(program)?;
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

fn program_to_dot<J: Jet>(program: &RedeemNode<J>) -> Result<String, Error> {
    let mut dot = String::new();
    writeln!(dot, "digraph {{\nranksep=3;")?;

    let (scribe_values, _) = compress::scribe_values_hidden(program);
    let reachable = scribe_reachable(program, &scribe_values);

    for item in program.post_order_iter::<MaxSharing<_>>() {
        let imr = item.node.data().imr();
        if !reachable.contains(&imr) {
            continue;
        }

        if let Some(value) = scribe_values.get(&imr) {
            fmt_scribe(&mut dot, value, item.index)?;
        } else {
            fmt_node(&mut dot, item)?;
        }
    }

    writeln!(&mut dot, "}}")?;
    Ok(dot)
}

/// Compute nodes that are reachable from root without entering scribe expressions
///
/// Nodes inside scribe expressions may be shared and thus reachable.
/// Therefore, the opposite approach of computing reachable nodes from scribe roots does not work.
pub fn scribe_reachable<J: Jet>(
    program: &RedeemNode<J>,
    scribe_values: &HashMap<Imr, Value>,
) -> HashSet<Imr> {
    let mut visited = HashSet::new();
    let mut stack = vec![program];

    while let Some(top) = stack.pop() {
        visited.insert(top.imr());

        if let Some(left) = top.left_child() {
            if !scribe_values.contains_key(&top.imr()) && !visited.contains(&left.imr()) {
                stack.push(left);
            }
        }
        if let Some(right) = top.right_child() {
            if !scribe_values.contains_key(&top.imr()) && !visited.contains(&right.imr()) {
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
    item: PostOrderIterItem<&RedeemNode<J>>,
) -> fmt::Result {
    write!(
        w,
        "{} [label=\"{}\\n{}\"];",
        item.index,
        item.node.inner(),
        item.node.arrow()
    )?;

    if let Some(i_abs) = item.left_index {
        if let Some(j_abs) = item.right_index {
            writeln!(w, "  {} -> {} [color=red];", item.index, i_abs)?;
            writeln!(w, "  {} -> {} [color=blue];", item.index, j_abs)?;
        } else {
            writeln!(w, "  {} -> {};", item.index, i_abs)?;
        }
    }

    Ok(())
}
