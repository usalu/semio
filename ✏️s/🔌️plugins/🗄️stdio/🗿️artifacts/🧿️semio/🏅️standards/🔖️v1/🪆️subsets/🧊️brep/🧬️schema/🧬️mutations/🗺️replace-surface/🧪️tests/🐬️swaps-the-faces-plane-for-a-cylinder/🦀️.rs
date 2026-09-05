//! 🧪️ `replace-surface` fixture — `🐬️swaps-the-faces-plane-for-a-cylinder`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: unknown face id ⇒ Error
//! `mutation.target-missing`, an identical surface ⇒ Warning `mutation.no-op`. The diff is a
//! `faces.modified` entry whose `BrepFaceDiff` sets `surface` and leaves `outer_loop`,
//! `inner_loops` AND `orientation` at `None` — three explicitly-untouched fields, the exact mirror
//! of `replace-curve`'s two.
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
    decode_semio_brep_snapshot_json(BEFORE).expect("replace-surface before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    decode_semio_brep_snapshot_json(AFTER).expect("replace-surface after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    decode_semio_brep_mutation_json(MUTATION).expect("replace-surface mutation decodes")
}

/// ▶️ The face's surface becomes a cylinder; its bounding loops and its sense stay put.
#[semio_framework_async_macros::async_test]
async fn replaces_the_surface_without_rebounding_the_face() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("replace-surface applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-surface/swaps-the-faces-plane-for-a-cylinder: applied state differs from the committed after-snapshot");
    let edited = &produced.faces[0];
    assert_ne!(edited.surface, base.faces[0].surface, "the surface really must have changed");
    assert_eq!(edited.outer_loop, base.faces[0].outer_loop, "replace-surface must not re-bound the face");
    assert_eq!(edited.orientation, base.faces[0].orientation, "replace-surface must not flip the face's sense");
    assert_eq!(produced.loops, base.loops, "replacing a surface must not rewrite a loop");
}

/// ↩️ The undo is a `replace-surface` carrying BASE's captured surface.
#[semio_framework_async_macros::async_test]
async fn the_undo_replace_surface_restores_the_original_plane() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "replace-surface of an existing face undoes as exactly one replace-surface");
    let SemioBrepMutation::ReplaceSurface(restore) = &undo[0] else { panic!("replace-surface must undo as replace-surface") };
    assert_eq!(restore.new_surface, base.faces[0].surface, "the undo must recapture BASE's own surface");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward replace-surface applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo replace-surface applies");
    }
    assert_eq!(current, base, "replace-surface/swaps-the-faces-plane-for-a-cylinder: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `BrepSurface` is internally tagged on `kind` and carries `rename_all_fields`, so both its VARIANTS and its members are camelCase (`halfAngle`, `majorRadius`, `uCount`), the spelling `📸️snapshot/🔣️.json` declares. This vector's `cylinder` arm has only single-word members, so it reads the same either way; the mismatch the missing `rename_all_fields` used to cause was invisible here and was found by the cross-language differential in `🧪️tests/⚓️mutate-semio-brep`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_semio_brep_snapshot_json(text).expect("snapshot decodes");
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        let original = pack::json::parse(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed {label} JSON is not canonical");
    }
    let reencoded = pack::json::from_dsl_value(&mutation().to_value());
    let original = pack::json::parse(MUTATION).expect("replace-surface mutation reparses");
    assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the face exists and the new surface genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome = pack::json::parse(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::json::Value::as_str), Some("applied"), "replace-surface/swaps-the-faces-plane-for-a-cylinder: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a surface with a genuinely different one must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `faces.modified`, and its per-face diff must carry `surface` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = pack::json::from_dsl_value(&outcome.diff().to_value());
    let committed = pack::json::parse(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-surface/swaps-the-faces-plane-for-a-cylinder: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed replace-surface diff decodes");

    let faces = decoded.faces.as_ref().expect("replace-surface must write the faces triple");
    assert!(faces.removed.is_empty() && faces.added.is_empty(), "a replace is a per-field modification, never a remove-and-re-add");
    assert_eq!(faces.modified.len(), 1, "exactly one face is modified");
    let face_diff = &faces.modified[0].diff;
    assert!(face_diff.outer_loop.is_none() && face_diff.inner_loops.is_none() && face_diff.orientation.is_none(), "the per-face diff must leave the loop and orientation fields unset");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    let original = pack::json::parse(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_semio_brep_diff_json(DIFF).expect("committed replace-surface diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-surface diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed diff did not carry before to after");
}
