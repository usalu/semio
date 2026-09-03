//! 🧪️ `delete-vertex` fixture — `removes-a-corner-vertex-and-cascades-into-its-two-incident-edges`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown id is Error `mutation.target-missing`;
//! otherwise the vertex is removed AND every edge whose `start_vertex` or `end_vertex` is that id
//! is removed too, with an INFO `mutation.cascade` naming them. The `edges` slot is set to `None`
//! when nothing cascaded — here two edges are incident, so the committed diff must carry BOTH
//! collections. The before-snapshot deliberately stops at edges: the leaf performs no loop/face
//! cascade, so a fixture carrying loops would encode a dangling reference the code never cleans.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{decode_semio_brep_diff_json, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{decode_semio_brep_mutation_json, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{decode_semio_brep_snapshot_json, SemioBrepSnapshot};
use pack::value::ToValue;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(BEFORE).expect("delete-vertex before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(AFTER).expect("delete-vertex after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    decode_semio_brep_mutation_json(MUTATION).expect("delete-vertex mutation decodes")
}

/// ▶️ `v2` and the two edges touching it disappear together.
#[semio_framework_async_macros::async_test]
async fn deletes_the_vertex_and_both_incident_edges() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-vertex applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: applied state differs from the committed after-snapshot");
    assert!(!produced.vertices.iter().any(|vertex| vertex.id == "v2"), "the named vertex must be gone");
    assert!(!produced.edges.iter().any(|edge| edge.start_vertex == "v2" || edge.end_vertex == "v2"), "no edge may keep a dangling reference to the deleted vertex");
    assert_eq!(produced.edges.len(), base.edges.len() - 2, "exactly the two incident edges are severed");
}

/// ↩️ The undo re-creates the vertex FIRST and then every severed edge, in base order.
#[semio_framework_async_macros::async_test]
async fn the_undo_recreates_the_vertex_then_both_severed_edges() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 3, "one create-vertex plus one create-edge per severed edge");
    assert!(matches!(undo[0], SemioBrepMutation::CreateVertex(_)), "the vertex must come back first — an edge without its endpoint would be dangling");
    assert!(matches!(undo[1], SemioBrepMutation::CreateEdge(_)) && matches!(undo[2], SemioBrepMutation::CreateEdge(_)), "both severed edges are re-created afterwards");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-vertex applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteVertex":{"id":"v2"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_semio_brep_snapshot_json(text).expect("snapshot decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        let original = pack::json::parse(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: committed {label} JSON is not canonical");
    }
    let reencoded = pack::json::from_dsl_value(&mutation().to_value());
    let original = pack::json::parse(MUTATION).expect("delete-vertex mutation reparses");
    assert_eq!(reencoded, original, "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the vertex exists, so mutation.target-missing must not fire; the cascade note is INFO and leaves the mutation applied
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome = pack::json::parse(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::json::Value::as_str), Some("applied"), "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: this case is declared applied");
    let produced = mutation().diff(&before());

    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "deleting a vertex that really severed edges raises exactly one message");
    assert_eq!(messages[0].code.0, "mutation.cascade", "the message must be the cascade note, not a rejection");
    assert_eq!(messages[0].level, protocol::Severity::Info, "a cascade note is INFO — the mutation still applies");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Two triples — `vertices.removed` and `edges.removed` — and no `modified`/`added` anywhere.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = pack::json::from_dsl_value(&outcome.diff().to_value());
    let committed = pack::json::parse(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-vertex diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-vertex diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: committed diff did not carry before to after");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collections the mutation is
/// allowed to touch appear in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-vertex diff decodes");

    let vertices = decoded.vertices.as_ref().expect("the vertices triple must be present");
    let edges = decoded.edges.as_ref().expect("the edges triple must be present — the cascade is part of the same diff");
    assert_eq!(vertices.removed, vec!["v2".to_string()], "the vertex is addressed by id");
    assert_eq!(edges.removed, vec!["e1".to_string(), "e2".to_string()], "the severed edges are listed in base order");
    assert!(decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "the leaf cascades into edges only — no loop/face/shell/solid slot may appear");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    let original = pack::json::parse(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges: committed diff JSON is not canonical");
}
