//! 🧪️ `rename-node` fixture — `🏷️renames`: the source node `a` takes the id `origin`, and the edge `e1` it starts follows.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose guards run in this order: an empty new id ⇒ FATAL `mutation.invariant`; no
//! such node ⇒ Error `mutation.target-missing`; the node's own id ⇒ Warning `mutation.no-op`; an id another node carries ⇒
//! FATAL `mutation.duplicate-id`.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-node/🏷️renames/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-node/🏷️renames/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-node/🏷️renames/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-node/🏷️renames/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️rename-node/🏷️renames/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rename-node fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("rename-node fixture parses")
}

/// ▶️ The node takes the id in place and the edge it starts names the new id; nothing else changes.
#[semio_framework_async_macros::async_test]
async fn renames_the_node_and_its_edge_endpoints() {
    let base = snapshot(BEFORE);
    let produced = decode::<SemioGraphMutation>(MUTATION).diff(&base).diff().apply(&base).expect("rename-node applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "rename-node/renames: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes.len(), base.nodes.len(), "a rename never adds or removes a node");
}

/// ↩️ The undo is one `rename-node` back to the old id, and it restores the before-snapshot (edges included).
#[semio_framework_async_macros::async_test]
async fn the_undo_renames_back() {
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    let undo = mutation.inverse(&base).expect("rename-node inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::RenameNode(_)]), "{undo:?}");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward rename-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo applies to the renamed graph");
    }
    assert_eq!(current, base, "rename-node/renames: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::schema::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "rename-node/renames: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "rename-node/renames: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and the new id is free, so no guard fires; the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "rename-node/renames: produced diff differs from the committed diff");
    assert_eq!(decode::<SemioGraphDiff>(DIFF).apply(&base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    use crate::standards::v1::subsets::graph::schema::mutations::rename_node::RenameNode;
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;
    let base = snapshot(BEFORE);
    let codes = |id: &str, new_id: &str| -> Vec<String> {
        SemioGraphMutation::RenameNode(RenameNode { id: GraphNodeId::new(id), new_id: GraphNodeId::new(new_id) }).diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect()
    };
    assert_eq!(codes("a", ""), ["mutation.invariant"]);
    assert_eq!(codes("ghost", "x"), ["mutation.target-missing"]);
    assert_eq!(codes("a", "a"), ["mutation.no-op"]);
    assert_eq!(codes("a", "b"), ["mutation.duplicate-id"]);
}
