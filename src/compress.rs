use std::collections::HashMap;

use simplicity::dag::{DagLike, MaxSharing};
use simplicity::jet::Jet;
use simplicity::node::Inner;
use simplicity::RedeemNode;
use simplicity::{Imr, Value};

/// Compute a mapping of nodes to the scribe expression that they represent.
/// This effectively reverses the function that turns scribe expressions into DAGs.
pub fn scribe_values<J: Jet>(program: &RedeemNode<J>) -> HashMap<Imr, Value> {
    let mut scribe_values = HashMap::new();

    for item in program.post_order_iter::<MaxSharing<_>>() {
        match item.node.inner() {
            Inner::Unit => {
                scribe_values.insert(item.node.imr(), Value::Unit);
            }
            Inner::InjL(left) => {
                if let Some(value) = scribe_values.get(&left.imr()) {
                    scribe_values.insert(item.node.imr(), Value::sum_l(value.clone()));
                }
            }
            Inner::InjR(left) => {
                if let Some(value) = scribe_values.get(&left.imr()) {
                    scribe_values.insert(item.node.imr(), Value::sum_r(value.clone()));
                }
            }
            Inner::Pair(left, right) => {
                if let Some(value_left) = scribe_values.get(&left.imr()) {
                    if let Some(value_right) = scribe_values.get(&right.imr()) {
                        scribe_values.insert(
                            item.node.imr(),
                            Value::prod(value_left.clone(), value_right.clone()),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    scribe_values
}
