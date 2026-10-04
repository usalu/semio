//! 🧪️ `set-edge-property` fixture — `🎚️sets`: the edge's existing `colour` property changes from `red` to `blue`.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-edge-property/🎚️sets/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-edge-property/🎚️sets/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-edge-property/🎚️sets/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-edge-property/🎚️sets/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-edge-property/🎚️sets/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-edge-property fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("set-edge-property fixture parses")
}

/// ▶️ The edge's properties change as declared; nodes and every other edge field are untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_before_snapshot() {
    let base = snapshot(BEFORE);
    let produced = decode::<SemioGraphMutation>(MUTATION).diff(&base).diff().apply(&base).expect("set-edge-property applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "set-edge-property/🎚️sets: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes, base.nodes, "an edge property edit must not disturb any node");
}

/// ↩️ The undo is one `set-edge-property` row and restores the byte-identical before-snapshot.
#[semio_framework_async_macros::async_test]
async fn the_undo_restores_the_before_snapshot_byte_for_byte() {
    use store::ArtifactPack;
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    let undo = mutation.inverse(&base).expect("set-edge-property inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::SetEdgeProperty(_)]), "{undo:?}");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward set-edge-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo applies to the edited graph");
    }
    assert_eq!(SemioGraphSnapshot::encode_pack(&current), SemioGraphSnapshot::encode_pack(&base), "set-edge-property/🎚️sets: the undo did not restore the before-snapshot bytes");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::schema::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "set-edge-property/🎚️sets: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "set-edge-property/🎚️sets: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: no guard fires and the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "set-edge-property/🎚️sets: produced diff differs from the committed diff");
    assert_eq!(decode::<SemioGraphDiff>(DIFF).apply(&base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    let base = snapshot(BEFORE);
    let codes = |leaf: SemioGraphMutation| -> Vec<String> { leaf.diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect() };
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphEdgeId;
    use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use crate::standards::v1::subsets::graph::schema::mutations::set_edge_property::SetEdgeProperty;
    let set = |edge: &str, key: &str, value: &str| SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: GraphEdgeId::new(edge), key: key.into(), value: SemioValue::Str { value: value.into() } });
    assert_eq!(codes(set("e1", "", "x")), ["mutation.invariant"]);
    assert_eq!(codes(set("ghost", "colour", "x")), ["mutation.target-missing"]);
    assert_eq!(codes(set("e1", "shape", "x")), ["mutation.target-missing"]);
    assert_eq!(codes(set("e1", "colour", "red")), ["mutation.no-op"]);
}
