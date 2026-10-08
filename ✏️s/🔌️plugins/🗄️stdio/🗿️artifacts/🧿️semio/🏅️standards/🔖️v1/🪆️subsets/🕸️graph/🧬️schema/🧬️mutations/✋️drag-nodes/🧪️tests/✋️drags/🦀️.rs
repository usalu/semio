//! 🧪️ `drag-nodes` fixture — `✋️drags`: both nodes of the two-node graph are dragged by one relative offset `(2.5, -1.5)`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose guards run in this order: empty or repeated targets ⇒ FATAL
//! `mutation.invariant`; non-finite offset ⇒ FATAL `mutation.invariant`; no target in the graph ⇒ Error
//! `mutation.target-missing`; a zero offset ⇒ Warning `mutation.no-op`. Otherwise every addressed node moves by the offset
//! from its BASE position. Every coordinate is dyadic, so the canonical-JSON assertions are exact.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-nodes/✋️drags/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-nodes/✋️drags/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-nodes/✋️drags/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-nodes/✋️drags/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-nodes/✋️drags/🎯️outcome/🔣️.json");

fn before() -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(BEFORE).expect("drag-nodes before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER).expect("drag-nodes after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("drag-nodes mutation decodes")
}

/// ▶️ Every dragged node moves by the offset from its base position; kinds, labels and edges do not change.
#[semio_framework_async_macros::async_test]
async fn drags_every_target_by_the_offset() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("drag-nodes applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "drag-nodes/drags: applied state differs from the committed after-snapshot");
    assert_eq!(produced.edges, base.edges, "dragging nodes must not disturb their edges");
}

/// ↩️ The undo is one absolute `move-node` per dragged node, back to BASE's own coordinates.
#[semio_framework_async_macros::async_test]
async fn the_undo_moves_every_node_back_to_its_base_position() {
    let base = before();
    let mutation = mutation();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let undo = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo.len(), 2, "two dragged nodes undo as two move-node rows");
    assert!(undo.iter().all(|step| matches!(step, SemioGraphMutation::MoveNode(_))));
    let mut current = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("forward drag-nodes applies");
    for step in undo.iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the undo move-node applies to the dragged graph");
    }
    assert_eq!(current, base, "drag-nodes/drags: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = crate::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::graph::io::text::snapshot::encode_semio_graph_snapshot_json(&decoded).expect("snapshot encodes")).expect("snapshot reparses");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "drag-nodes/drags: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("drag-nodes mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("drag-nodes mutation reparses"), "drag-nodes/drags: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: both targets exist and the offset is finite and non-zero, so no guard fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    assert!(mutation().diff(&before()).messages().is_empty(), "both targets exist and the offset is finite and non-zero");
}

/// 🔺️ The produced delta equals the committed diff, which carries the whole rebuilt node list and no edges.
#[semio_framework_async_macros::async_test]
async fn produces_and_applies_the_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "drag-nodes/drags: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: SemioGraphDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed drag-nodes diff decodes");
    assert!(decoded.edges.is_none(), "drag-nodes must leave the edges slot untouched");
    assert_eq!(protocol::apply_diff(&decoded, &base).expect("committed drag-nodes diff applies"), expected_after(), "drag-nodes/drags: committed diff did not carry before to after");
}

/// 🚧️ The guard branches: repeated targets and a non-finite offset are Fatal `mutation.invariant`, a target-less drag is
/// `mutation.target-missing`, a stranger among real targets is `mutation.partial`, a zero offset is `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    use crate::standards::v1::subsets::graph::schema::mutations::drag_nodes::DragNodes;
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;
    let base = before();
    let codes = |targets: &[&str], dx: f64, dy: f64| -> Vec<String> {
        let drag = SemioGraphMutation::DragNodes(DragNodes { targets: targets.iter().map(|id| GraphNodeId::new(*id)).collect(), dx, dy });
        drag.diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect()
    };
    assert_eq!(codes(&["a", "a"], 1.0, 1.0), ["mutation.invariant"]);
    assert_eq!(codes(&["a"], f64::NAN, 1.0), ["mutation.invariant"]);
    assert_eq!(codes(&["ghost"], 1.0, 1.0), ["mutation.target-missing"]);
    assert_eq!(codes(&["a", "ghost"], 1.0, 1.0), ["mutation.partial"]);
    assert_eq!(codes(&["a"], 0.0, 0.0), ["mutation.no-op"]);
}
