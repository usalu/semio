//! 🧪️ `📄set-snapshot` fixture — `relabels-and-repositions-the-transform-node`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔁️ The case renames the `transform` node and drags it on the canvas. Its `kind`, its `factor`
//! param, the whole `source` node and the `e1` edge are all identical across the change — so
//! `SemioFlowDiff` must carry an ID-keyed `nodes` triple modifying key `"b"` alone, with `edges`
//! absent. The DAG's collections are keyed by `id`, never by position, precisely so a relabel
//! never reads as a remove-plus-add.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::diff::SemioFlowDiff;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{apply_semio_flow_mutation, SemioFlowMutation};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioFlowSnapshot {
    serde_json::from_str(BEFORE).expect("before flow snapshot decodes")
}
fn expected_after() -> SemioFlowSnapshot {
    serde_json::from_str(AFTER).expect("after flow snapshot decodes")
}
fn mutation() -> SemioFlowMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the two-node DAG to exactly the committed `after`: node `b` is renamed
/// and moved.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_flow_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-flow/set-snapshot: a genuinely changed graph must not raise any message");
    assert_eq!(snapshot.nodes[1].id, "b", "semio-flow/set-snapshot: a node's id is its identity and is never rewritten by a patch");
    assert_eq!(snapshot.nodes[1].label, "Scale Twice", "semio-flow/set-snapshot: the transform node must be relabelled");
    assert_eq!(snapshot.nodes[1].position, SemioPoint2 { x: 120.0, y: 40.0 }, "semio-flow/set-snapshot: the transform node must be repositioned");
    assert_eq!(snapshot.edges, before().edges, "semio-flow/set-snapshot: moving a node must not rewire any edge");
    assert_eq!(snapshot, expected_after(), "semio-flow/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore both the old label
/// and the old canvas position.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_flow_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_flow_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-flow/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed graphs and the mutation are already canonical: a node's `params` list is
/// always written (its `Vec` carries no `skip_serializing_if`), and an edge endpoint is a `PortRef`
/// object with `node`/`port`, never a flattened `"a:out"` string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioFlowSnapshot = serde_json::from_str(text).expect("flow snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("flow snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("flow snapshot reparses");
        assert_eq!(reencoded, original, "semio-flow/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-flow/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the graph really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-flow/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_flow_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-flow/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-flow/set-snapshot: an applied set-snapshot must actually move the graph");
}

/// 🔺️ The sparse `SemioFlowDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: node `a` must not appear, `edges` must stay absent, and the patch on
/// node `b` must set `label`/`position` only, leaving `kind` and the `params` triple unset.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced flow diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed flow diff decodes");
    assert_eq!(produced, committed, "semio-flow/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioFlowDiff`: an ID-keyed `nodes`
/// triple whose single `modified` entry is keyed `"b"`, with nothing removed and nothing added.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioFlowDiff = serde_json::from_str(DIFF).expect("committed flow diff decodes");
    assert!(decoded.edges.is_none(), "semio-flow/set-snapshot: the edge collection must stay untouched");
    let nodes = decoded.nodes.as_ref().expect("the committed diff carries a nodes triple");
    assert!(nodes.removed.is_empty() && nodes.added.is_empty() && nodes.modified.len() == 1 && nodes.modified[0].key == "b", "semio-flow/set-snapshot: exactly node b may be patched, addressed by id");
    let patch = &nodes.modified[0].diff;
    assert!(patch.kind.is_none() && patch.params.is_none(), "semio-flow/set-snapshot: the node's kind and params did not move and must stay absent");
    let reencoded = serde_json::to_value(&decoded).expect("flow diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed flow diff reparses");
    assert_eq!(reencoded, original, "semio-flow/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — label plus
/// position is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioFlowDiff = serde_json::from_str(DIFF).expect("committed flow diff decodes");
    let produced = <SemioFlowDiff as protocol::MutationDiff<SemioFlowSnapshot>>::apply(&decoded, &before()).expect("committed flow diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-flow/set-snapshot: committed diff did not carry before to after");
}
