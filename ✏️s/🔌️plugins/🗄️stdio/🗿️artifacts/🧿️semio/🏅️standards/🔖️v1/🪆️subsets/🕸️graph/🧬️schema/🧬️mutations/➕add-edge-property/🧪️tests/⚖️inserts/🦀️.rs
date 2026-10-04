//! 🧪️ `add-edge-property` fixture — `⚖️inserts`: a `weight` property is attached AHEAD of the edge's `colour` property.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕add-edge-property/⚖️inserts/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕add-edge-property/⚖️inserts/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕add-edge-property/⚖️inserts/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕add-edge-property/⚖️inserts/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕add-edge-property/⚖️inserts/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("add-edge-property fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("add-edge-property fixture parses")
}

/// ▶️ The edge's properties change as declared; nodes and every other edge field are untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_before_snapshot() {
    let base = snapshot(BEFORE);
    let produced = decode::<SemioGraphMutation>(MUTATION).diff(&base).diff().apply(&base).expect("add-edge-property applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "add-edge-property/⚖️inserts: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes, base.nodes, "an edge property edit must not disturb any node");
}

/// ↩️ The undo is one `remove-edge-property` row and restores the byte-identical before-snapshot.
#[semio_framework_async_macros::async_test]
async fn the_undo_restores_the_before_snapshot_byte_for_byte() {
    use store::ArtifactPack;
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    let undo = mutation.inverse(&base).expect("add-edge-property inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::RemoveEdgeProperty(_)]), "{undo:?}");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-edge-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo applies to the edited graph");
    }
    assert_eq!(SemioGraphSnapshot::encode_pack(&current), SemioGraphSnapshot::encode_pack(&base), "add-edge-property/⚖️inserts: the undo did not restore the before-snapshot bytes");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::schema::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "add-edge-property/⚖️inserts: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "add-edge-property/⚖️inserts: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: no guard fires and the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "add-edge-property/⚖️inserts: produced diff differs from the committed diff");
    assert_eq!(decode::<SemioGraphDiff>(DIFF).apply(&base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    let base = snapshot(BEFORE);
    let codes = |leaf: SemioGraphMutation| -> Vec<String> { leaf.diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect() };
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphEdgeId;
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
    use crate::standards::v1::subsets::graph::schema::mutations::add_edge_property::AddEdgeProperty;
    let add = |edge: &str, key: &str| SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: GraphEdgeId::new(edge), index: 0, property: SemioValueEntry { key: key.into(), value: SemioValue::Null } });
    assert_eq!(codes(add("e1", "")), ["mutation.invariant"]);
    assert_eq!(codes(add("ghost", "weight")), ["mutation.target-missing"]);
    assert_eq!(codes(add("e1", "colour")), ["mutation.no-op"]);
}
