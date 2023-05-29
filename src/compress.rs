use simplicity::core::iter::DagIterable;
use simplicity::core::redeem::RedeemNodeInner;
use simplicity::core::{redeem, RedeemNode, Value};
use simplicity::jet::Jet;
use std::collections::HashMap;

/// Compute a mapping of nodes to the scribe expression that they represent.
/// This effectively reverses the function that turns scribe expressions into DAGs.
pub fn compress_scribe<J: Jet>(program: &RedeemNode<J>) -> HashMap<redeem::RefWrapper<J>, Value> {
    let mut node_to_scribe = HashMap::new();

    for node in redeem::RefWrapper(program).iter_post_order() {
        match &node.0.inner {
            RedeemNodeInner::Unit => {
                node_to_scribe.insert(node, Value::Unit);
            }
            RedeemNodeInner::InjL(_) => {
                let left = node.get_left().unwrap();
                if let Some(value) = node_to_scribe.get(&left) {
                    node_to_scribe.insert(node, Value::sum_l(value.clone()));
                }
            }
            RedeemNodeInner::InjR(_) => {
                let left = node.get_left().unwrap();
                if let Some(value) = node_to_scribe.get(&left) {
                    node_to_scribe.insert(node, Value::sum_r(value.clone()));
                }
            }
            RedeemNodeInner::Pair(_, _) => {
                let left = node.get_left().unwrap();
                let right = node.get_right().unwrap();

                if let Some(value_left) = node_to_scribe.get(&left) {
                    if let Some(value_right) = node_to_scribe.get(&right) {
                        node_to_scribe
                            .insert(node, Value::prod(value_left.clone(), value_right.clone()));
                    }
                }
            }
            _ => {}
        }
    }

    node_to_scribe
}
