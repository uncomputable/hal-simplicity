use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use simplicity::dag::{DagLike, MaxSharing, SharingTracker};
use simplicity::node::{Inner, Marker, Node};
use simplicity::Value;

/// Compute a mapping of scribe expressions to the value that they encode.
///
/// This effectively reverses the function scribe that turns values into expressions.
fn scribe_values<N: Marker>(program: &Node<N>) -> HashMap<N::SharingId, Arc<Value>> {
    let mut scribe_values = HashMap::new();

    for item in program.post_order_iter::<MaxSharing<_>>() {
        let id = match item.node.sharing_id() {
            Some(id) => id,
            None => continue, // expressions that include witness or disconnect never encode scribe
        };

        match item.node.inner() {
            Inner::Unit => {
                scribe_values.insert(id, Value::unit());
            }
            Inner::InjL(left) => {
                let left_id = left.sharing_id().expect("parent has id");
                if let Some(value) = scribe_values.get(&left_id) {
                    scribe_values.insert(id, Value::sum_l(value.clone()));
                }
            }
            Inner::InjR(left) => {
                let left_id = left.sharing_id().expect("parent has id");
                if let Some(value) = scribe_values.get(&left_id) {
                    scribe_values.insert(id, Value::sum_r(value.clone()));
                }
            }
            Inner::Pair(left, right) => {
                let left_id = left.sharing_id().expect("parent has id");
                if let Some(value_left) = scribe_values.get(&left_id) {
                    let right_id = right.sharing_id().expect("parent has id");
                    if let Some(value_right) = scribe_values.get(&right_id) {
                        scribe_values
                            .insert(id, Value::prod(value_left.clone(), value_right.clone()));
                    }
                }
            }
            Inner::Word(value) => {
                scribe_values.insert(id, value.clone());
            }
            _ => {}
        }
    }

    scribe_values
}

/// Compute a mapping of scribe expressions to the value that they encode.
/// These expressions are maximal, so one expression is never contained in another expression.
///
/// Also return the set of subexpressions that are hidden inside a larger scribe.
pub fn scribe_values_hidden<N: Marker>(
    program: &Node<N>,
) -> (HashMap<N::SharingId, Arc<Value>>, HashSet<N::SharingId>) {
    let scribe_values = scribe_values(program);
    let mut top_scribe_values = HashMap::new();
    let mut scribe_hidden: HashSet<_> = scribe_values.keys().cloned().collect();
    let mut stack = vec![program];

    while let Some(top) = stack.pop() {
        if let Some(id) = top.sharing_id() {
            if let Some(value) = scribe_values.get(&id) {
                top_scribe_values.insert(id.clone(), value.clone());
                scribe_hidden.remove(&id);
                continue;
            }
        }

        if let Some(right) = top.right_child() {
            stack.push(right);
        }
        if let Some(left) = top.left_child() {
            stack.push(left);
        }
    }

    (top_scribe_values, scribe_hidden)
}

/// Sharing with compressed sharing nodes:
///
/// If a subgraph encodes scribe, then only the root is visible and the nodes below are invisible.
/// For all other nodes, the DAG is shared according to the tracker `T`.
pub struct CompressScribe<N: Marker, T> {
    tracker: T,
    scribe_hidden: HashSet<N::SharingId>,
}

impl<N: Marker, T: Default> CompressScribe<N, T> {
    /// Create a new sharing tracker from the set of subexpressions that are hidden inside scribe.
    pub fn new(scribe_hidden: HashSet<N::SharingId>) -> Self {
        Self {
            tracker: T::default(),
            scribe_hidden,
        }
    }
}

impl<'a, N: Marker, T: SharingTracker<&'a Node<N>>> SharingTracker<&'a Node<N>>
    for CompressScribe<N, T>
{
    fn record(&mut self, d: &&'a Node<N>, index: usize) -> Option<usize> {
        match self.tracker.record(d, index) {
            Some(index) => Some(index),
            None => {
                if d.sharing_id()
                    .is_some_and(|id| self.scribe_hidden.contains(&id))
                {
                    Some(index)
                } else {
                    None
                }
            }
        }
    }

    fn seen_before(&self, d: &&'a Node<N>) -> Option<usize> {
        self.tracker.seen_before(d)
    }
}
