use simplicity::core::redeem::RedeemNodeInner;
use simplicity::core::RedeemNode;
use simplicity::dag::{DagLike, FullSharing};
use simplicity::jet::Jet;
use simplicity::{Imr, Value};
use std::collections::HashMap;

/// Compute a mapping of nodes to the scribe expression that they represent.
/// This effectively reverses the function that turns scribe expressions into DAGs.
pub fn compress_scribe<J: Jet>(program: &RedeemNode<J>) -> HashMap<Imr, Value> {
    let mut node_to_scribe = HashMap::new();

    for data in program.post_order_iter::<FullSharing>() {
        match &data.node.inner {
            RedeemNodeInner::Unit => {
                node_to_scribe.insert(data.node.imr, Value::Unit);
            }
            RedeemNodeInner::InjL(_) => {
                let left = data.node.get_left().unwrap();
                if let Some(value) = node_to_scribe.get(&left.imr) {
                    node_to_scribe.insert(data.node.imr, Value::sum_l(value.clone()));
                }
            }
            RedeemNodeInner::InjR(_) => {
                let left = data.node.get_left().unwrap();
                if let Some(value) = node_to_scribe.get(&left.imr) {
                    node_to_scribe.insert(data.node.imr, Value::sum_r(value.clone()));
                }
            }
            RedeemNodeInner::Pair(_, _) => {
                let left = data.node.get_left().unwrap();
                let right = data.node.get_right().unwrap();

                if let Some(value_left) = node_to_scribe.get(&left.imr) {
                    if let Some(value_right) = node_to_scribe.get(&right.imr) {
                        node_to_scribe.insert(
                            data.node.imr,
                            Value::prod(value_left.clone(), value_right.clone()),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    node_to_scribe
}
