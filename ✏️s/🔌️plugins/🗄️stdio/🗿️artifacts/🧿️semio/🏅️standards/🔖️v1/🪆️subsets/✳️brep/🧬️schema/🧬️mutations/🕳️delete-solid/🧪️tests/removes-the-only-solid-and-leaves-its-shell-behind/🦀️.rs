//! 🧪️ `delete-solid` fixture — `removes-the-only-solid-and-leaves-its-shell-behind`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown solid id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `solids.removed[id]`. A solid is the
//! TOP of this artifact's topology, so there is nothing above it to cascade into, and the leaf
//! deliberately does not cascade down into its shells either.
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
    decode_semio_brep_snapshot_json(BEFORE).expect("delete-solid before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(AFTER).expect("delete-solid after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    decode_semio_brep_mutation_json(MUTATION).expect("delete-solid mutation decodes")
}

/// ▶️ The solid goes; the shell it bounded and everything under it remain.
#[semio_framework_async_macros::async_test]
async fn removes_the_solid_and_leaves_the_whole_topology_under_it() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-solid applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: applied state differs from the committed after-snapshot");
    assert!(produced.solids.is_empty(), "the only solid must be gone");
    assert_eq!(produced.shells, base.shells, "delete-solid must NOT cascade down into the shells it bounded");
    assert_eq!((produced.vertices, produced.faces), (base.vertices, base.faces), "nothing further down the topology may move either");
}

/// ↩️ The undo re-creates the solid with its captured shell list, void flags included.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_solid_restores_the_captured_shell_list() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-solid of an existing solid undoes as exactly one create-solid");
    let SemioBrepMutation::CreateSolid(recreate) = &undo[0] else { panic!("delete-solid must undo as create-solid") };
    assert_eq!(recreate.shells, base.solids[0].shells, "the undo must recapture the deleted solid's own shell list verbatim, is_void flags included");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-solid applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-solid applies");
    }
    assert_eq!(current, base, "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteSolid":{"id":"so1"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_semio_brep_snapshot_json(text).expect("snapshot decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        let original = pack::json::parse(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: committed {label} JSON is not canonical");
    }
    let reencoded = pack::json::from_dsl_value(&mutation().to_value());
    let original = pack::json::parse(MUTATION).expect("delete-solid mutation reparses");
    assert_eq!(reencoded, original, "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the solid exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome = pack::json::parse(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::json::Value::as_str), Some("applied"), "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing solid must raise no diagnostics — nothing references a solid from above");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `solids.removed`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = pack::json::from_dsl_value(&outcome.diff().to_value());
    let committed = pack::json::parse(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-solid diff decodes");

    let solids = decoded.solids.as_ref().expect("delete-solid must write the solids triple");
    assert_eq!(solids.removed, vec!["so1".to_string()], "the removal is addressed by solid id");
    assert!(solids.modified.is_empty() && solids.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.faces.is_none() && decoded.shells.is_none(), "delete-solid cascades nowhere — no other collection may appear");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    let original = pack::json::parse(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed delete-solid diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-solid diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-solid/removes-the-only-solid-and-leaves-its-shell-behind: committed diff did not carry before to after");
}
