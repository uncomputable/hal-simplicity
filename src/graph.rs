use std::fmt;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IOWrite;
use std::path::Path;

use layout::backends::svg::SVGWriter;
use layout::gv::{DotParser, GraphBuilder};
use simplicity::dag::{DagLike, MaxSharing, PostOrderIterItem};
use simplicity::jet::Jet;
use simplicity::{CommitNode, Value};

use crate::compress;
use crate::compress::CompressScribe;
use crate::error::Error;

pub fn visualize<J: Jet>(program: &CommitNode<J>) -> Result<(), Error> {
    let dot = program_to_dot(program)?;
    println!("{}", dot);
    Ok(())
}

#[allow(dead_code)]
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

fn program_to_dot<J: Jet>(program: &CommitNode<J>) -> Result<String, Error> {
    let mut dot = String::new();
    writeln!(dot, "digraph {{\nranksep=3;")?;

    let (scribe_values, scribe_hidden) = compress::scribe_values_hidden(program);
    let tracker = CompressScribe::<_, MaxSharing<_>>::new(scribe_hidden);

    for item in program.post_order_iter_with_tracker(tracker) {
        if let Some(value) = item.node.sharing_id().and_then(|i| scribe_values.get(&i)) {
            fmt_scribe(&mut dot, value, item.index)?;
        } else {
            fmt_node(&mut dot, item)?;
        }
    }

    writeln!(&mut dot, "}}")?;
    Ok(dot)
}

fn fmt_scribe<W: FmtWrite>(w: &mut W, value: &Value, index: usize) -> fmt::Result {
    let (bytes, bit_len) = value.to_bytes_len();

    write!(w, "{} [label=\"", index)?;

    match bit_len {
        0 => {
            write!(w, "unit")?;
        }
        n if n % 8 == 0 => {
            for byte in &bytes {
                write!(w, "{:02X}", byte)?;
            }
        }
        _ => {
            let mut bits_printed = 0;
            'outer: for byte in &bytes {
                for i in (0..8).rev() {
                    if bits_printed >= bit_len {
                        break 'outer;
                    }
                    write!(w, "{}", (byte >> i) & 1)?;
                    bits_printed += 1;
                }
            }
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
    item: PostOrderIterItem<&CommitNode<J>>,
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
