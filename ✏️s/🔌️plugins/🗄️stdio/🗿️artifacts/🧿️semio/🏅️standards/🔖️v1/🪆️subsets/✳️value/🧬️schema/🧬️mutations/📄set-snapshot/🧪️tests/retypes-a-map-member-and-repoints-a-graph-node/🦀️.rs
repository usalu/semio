//! 🧪️ `📄set-snapshot` fixture — `retypes-a-map-member-and-repoints-a-graph-node`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🌳 The case changes the root map's `count` member from an `Int` lexeme to a `Float` lexeme — a
//! KIND change, which `value_diff_between` answers with `SemioValueDiff::Replace` rather than a
//! same-kind field patch — and separately rewrites the `Str` payload of graph node `n-1`, which
//! IS a same-kind patch. The `title` member and the `alias` `Ref` are untouched, so neither may
//! appear anywhere in the delta. Both halves matter: `root` and `nodes` are independent slots on
//! `SemioValueTreeDiff`, and this fixture is what proves a set-snapshot fills exactly the ones
//! that moved.

use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{SemioValueDiff, SemioValueTreeDiff};
use crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::{apply_semio_value_mutation, SemioValueMutation};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioValueSnapshot {
    serde_json::from_str(BEFORE).expect("before value-graph snapshot decodes")
}
fn expected_after() -> SemioValueSnapshot {
    serde_json::from_str(AFTER).expect("after value-graph snapshot decodes")
}
fn mutation() -> SemioValueMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the value graph to exactly the committed `after`: a float `count` and
/// a finalized node payload.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_value_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-value/set-snapshot: a genuinely changed graph must not raise any message");
    assert_eq!(snapshot.nodes.len(), 1, "semio-value/set-snapshot: the id-keyed backing store must keep its single node");
    assert_eq!(snapshot, expected_after(), "semio-value/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must turn `count` back into an
/// `Int` (a second kind change, in the other direction) and restore the node's draft payload.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_value_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_value_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-value/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed graphs and the mutation are already canonical: `SemioValue` is internally
/// tagged on `kind` with STRUCT variants only, `Int`/`Float` keep their original source lexeme as
/// a string rather than a JSON number (arbitrary precision is the whole point), and `ValueId` is a
/// named single-field struct, never a bare newtype.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioValueSnapshot = serde_json::from_str(text).expect("value-graph snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("value-graph snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("value-graph snapshot reparses");
        assert_eq!(reencoded, original, "semio-value/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-value/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the graph really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-value/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_value_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-value/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-value/set-snapshot: an applied set-snapshot must actually move the graph");
}

/// 🔺️ The sparse `SemioValueTreeDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the root delta must descend through the name-keyed map triple to the
/// single changed member (never a whole-root `Replace`), while the node delta must reach node
/// `n-1` by ID and not renumber the backing store.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced value-graph diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed value-graph diff decodes");
    assert_eq!(produced, committed, "semio-value/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioValueTreeDiff`: a `Map` delta at
/// the root whose one modified member is a `Replace` (the Int→Float kind change), and a name-keyed
/// `nodes` triple with neither removals nor additions.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioValueTreeDiff = serde_json::from_str(DIFF).expect("committed value-graph diff decodes");
    let SemioValueDiff::Map { diff: map } = decoded.root.as_ref().expect("the committed diff patches the root value") else {
        panic!("semio-value/set-snapshot: the root delta must stay a structural Map diff — only the member inside it changed kind");
    };
    assert!(map.removed.is_empty() && map.added.is_empty() && map.modified.len() == 1 && map.modified[0].key == "count", "semio-value/set-snapshot: exactly the count member may be reported, and no member may be dropped or introduced");
    assert!(matches!(map.modified[0].diff, SemioValueDiff::Replace { .. }), "semio-value/set-snapshot: an Int→Float kind change must lower to Replace, never to a same-kind Float patch");
    let nodes = decoded.nodes.as_ref().expect("the committed diff carries a nodes triple");
    assert!(nodes.removed.is_empty() && nodes.added.is_empty() && nodes.modified.len() == 1, "semio-value/set-snapshot: the graph node must be patched by id, never removed and re-added");
    let reencoded = serde_json::to_value(&decoded).expect("value-graph diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed value-graph diff reparses");
    assert_eq!(reencoded, original, "semio-value/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the root and
/// nodes deltas together are a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioValueTreeDiff = serde_json::from_str(DIFF).expect("committed value-graph diff decodes");
    let produced = <SemioValueTreeDiff as protocol::MutationDiff<SemioValueSnapshot>>::apply(&decoded, &before()).expect("committed value-graph diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-value/set-snapshot: committed diff did not carry before to after");
}
