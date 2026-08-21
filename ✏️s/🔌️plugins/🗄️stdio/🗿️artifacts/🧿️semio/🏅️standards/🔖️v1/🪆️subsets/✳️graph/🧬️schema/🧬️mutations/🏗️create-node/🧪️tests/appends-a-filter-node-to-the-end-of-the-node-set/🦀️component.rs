//! 🧪️ `create-node` fixture — `appends-a-filter-node-to-the-end-of-the-node-set`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a duplicate `GraphNodeId` is FATAL
//! `mutation.duplicate-id`; otherwise the fully-specified node is PUSHED — `nodes` is an id-keyed
//! set with no display order, so there is no index in the payload and the new node always lands
//! last. `edges` stays `None`: creating an isolated node touches no edge.
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioGraphSnapshot {
    serde_json::from_str(BEFORE).expect("create-node before snapshot decodes")
}
fn expected_after() -> SemioGraphSnapshot {
    serde_json::from_str(AFTER).expect("create-node after snapshot decodes")
}
fn mutation() -> SemioGraphMutation {
    serde_json::from_str(MUTATION).expect("create-node mutation decodes")
}

/// ▶️ The new node is appended with every field the payload carried, and the existing two are
/// untouched.
#[semio_framework_async_macros::async_test]
async fn appends_the_filter_node_with_its_full_payload() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-node applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-a-filter-node-to-the-end-of-the-node-set: applied state differs from the committed after-snapshot");
    assert_eq!(produced.nodes.len(), base.nodes.len() + 1, "create-node adds exactly one node");
    let created = produced.nodes.last().expect("the created node is the last one — an id-keyed set has no insertion index");
    assert_eq!(created.id.value, "c", "the node keeps the id the payload named");
    assert_eq!(created.kind, "filter", "create-node carries the full initial payload, kind included");
    assert_eq!(created.label, "Filter", "create-node carries the full initial payload, label included");
    assert_eq!(&produced.nodes[..base.nodes.len()], &base.nodes[..], "the pre-existing nodes must be byte-identical and keep their order");
    assert_eq!(produced.edges, base.edges, "creating an isolated node must not touch any edge");
}

/// ↩️ `create-node`'s undo is a single `delete-node` addressing the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_node_removes_the_filter_node_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "creating a node with no edges undoes as exactly one delete-node");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-node applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-node applies to the widened graph");
    }
    assert_eq!(current, base, "create-node/appends-a-filter-node-to-the-end-of-the-node-set: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `GraphNodeId` is a NAMED single-field struct, so
/// an id encodes as `{"value":"c"}`, never as a bare string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioGraphSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/appends-a-filter-node-to-the-end-of-the-node-set: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-node mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-node mutation reparses");
    assert_eq!(reencoded, original, "create-node/appends-a-filter-node-to-the-end-of-the-node-set: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no node with id `c` exists, so the FATAL `mutation.duplicate-id` branch
/// must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_duplicate_id_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-node/appends-a-filter-node-to-the-end-of-the-node-set: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a node with a fresh id must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-node/appends-a-filter-node-to-the-end-of-the-node-set: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and omits `edges` entirely.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_edges_entirely() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed create-node diff decodes");
    assert!(decoded.edges.is_none(), "create-node must leave the edges slot untouched");
    assert_eq!(decoded.nodes.as_ref().map(|list| list.values.len()), Some(3), "the diff must carry all three nodes of the final set");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("edges").is_none(), "the committed diff JSON must not carry an edges key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "create-node/appends-a-filter-node-to-the-end-of-the-node-set: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioGraphDiff = serde_json::from_str(DIFF).expect("committed create-node diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-node diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-a-filter-node-to-the-end-of-the-node-set: committed diff did not carry before to after");
}
