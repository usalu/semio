//! 🧪️ `remove-edge-property` fixture — `➖️detaches`: the edge's trailing `weight` property is detached.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-edge-property/➖️detaches/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-edge-property/➖️detaches/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-edge-property/➖️detaches/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-edge-property/➖️detaches/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-edge-property/➖️detaches/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("remove-edge-property fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("remove-edge-property fixture parses")
}

/// ▶️ The edge's properties change as declared; nodes and every other edge field are untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_before_snapshot() {
    let base = snapshot(BEFORE);
    let produced = decode::<SemioGraphMutation>(MUTATION).diff(&base).diff().apply(&base).expect("remove-edge-property applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "remove-edge-property/➖️detaches: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes, base.nodes, "an edge property edit must not disturb any node");
}

/// ↩️ The undo is one `add-edge-property` row and restores the byte-identical before-snapshot.
#[semio_framework_async_macros::async_test]
async fn the_undo_restores_the_before_snapshot_byte_for_byte() {
    use store::ArtifactPack;
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    let undo = mutation.inverse(&base).expect("remove-edge-property inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::AddEdgeProperty(_)]), "{undo:?}");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-edge-property applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo applies to the edited graph");
    }
    assert_eq!(SemioGraphSnapshot::encode_pack(&current), SemioGraphSnapshot::encode_pack(&base), "remove-edge-property/➖️detaches: the undo did not restore the before-snapshot bytes");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::io::text::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "remove-edge-property/➖️detaches: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "remove-edge-property/➖️detaches: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: no guard fires and the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "remove-edge-property/➖️detaches: produced diff differs from the committed diff");
    assert_eq!(decode::<SemioGraphDiff>(DIFF).apply(&base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    let base = snapshot(BEFORE);
    let codes = |leaf: SemioGraphMutation| -> Vec<String> { leaf.diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect() };
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphEdgeId;
    use crate::standards::v1::subsets::graph::schema::mutations::remove_edge_property::RemoveEdgeProperty;
    let remove = |edge: &str, key: &str| SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: GraphEdgeId::new(edge), key: key.into() });
    assert_eq!(codes(remove("ghost", "weight")), ["mutation.target-missing"]);
    assert_eq!(codes(remove("e1", "absent")), ["mutation.target-missing"]);
}
