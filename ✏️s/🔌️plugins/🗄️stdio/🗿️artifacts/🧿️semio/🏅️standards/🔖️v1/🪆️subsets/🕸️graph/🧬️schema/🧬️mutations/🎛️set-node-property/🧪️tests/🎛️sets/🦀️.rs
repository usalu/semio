//! 🧪️ `set-node-property` fixture — `🎛️sets`: the source node's existing `colour` property changes from `red` to `blue`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose guards run in this order: empty key ⇒ FATAL `mutation.invariant`; no such
//! node or no such key ⇒ Error `mutation.target-missing`; an equal value ⇒ Warning `mutation.no-op`. Otherwise the property
//! takes the value in place, keeping its position among the node's properties.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️set-node-property/🎛️sets/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️set-node-property/🎛️sets/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️set-node-property/🎛️sets/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️set-node-property/🎛️sets/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️set-node-property/🎛️sets/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-node-property fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("set-node-property fixture parses")
}

/// ▶️ The property takes the value in place; every other field, property and edge is untouched.
#[semio_framework_async_macros::async_test]
async fn sets_the_property_value_in_place() {
    let base = snapshot(BEFORE);
    let produced = decode::<SemioGraphMutation>(MUTATION).diff(&base).diff().apply(&base).expect("set-node-property applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "set-node-property/sets: applied state differs from the committed after-snapshot");
    assert_eq!(produced.edges, base.edges, "setting a property must not disturb any edge");
}

/// ↩️ The undo is one `set-node-property` back to the base value, and it restores the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn the_undo_sets_the_base_value_back() {
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    let undo = mutation.inverse(&base).expect("set-node-property inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::SetNodeProperty(_)]), "{undo:?}");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward set-node-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo applies to the edited graph");
    }
    assert_eq!(current, base, "set-node-property/sets: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::schema::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "set-node-property/sets: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "set-node-property/sets: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: node and key exist and the value differs, so no guard fires; the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "set-node-property/sets: produced diff differs from the committed diff");
    assert_eq!(decode::<SemioGraphDiff>(DIFF).apply(&base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    use crate::standards::v1::subsets::graph::schema::mutations::set_node_property::SetNodeProperty;
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;
    use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
    let base = snapshot(BEFORE);
    let codes = |node: &str, key: &str, value: &str| -> Vec<String> {
        let leaf = SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: GraphNodeId::new(node), key: key.into(), value: SemioValue::Str { value: value.into() } });
        leaf.diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect()
    };
    assert_eq!(codes("a", "", "x"), ["mutation.invariant"]);
    assert_eq!(codes("ghost", "colour", "x"), ["mutation.target-missing"]);
    assert_eq!(codes("a", "shape", "x"), ["mutation.target-missing"]);
    assert_eq!(codes("a", "colour", "red"), ["mutation.no-op"]);
}
