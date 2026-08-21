//! 🧪️ `replace-surface` fixture — `swaps-the-faces-plane-for-a-cylinder`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: unknown face id ⇒ Error
//! `mutation.target-missing`, an identical surface ⇒ Warning `mutation.no-op`. The diff is a
//! `faces.modified` entry whose `BrepFaceDiff` sets `surface` and leaves `outer_loop`,
//! `inner_loops` AND `orientation` at `None` — three explicitly-untouched fields, the exact mirror
//! of `replace-curve`'s two.
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioBrepSnapshot {
    serde_json::from_str(BEFORE).expect("replace-surface before snapshot decodes")
}
fn expected_after() -> SemioBrepSnapshot {
    serde_json::from_str(AFTER).expect("replace-surface after snapshot decodes")
}
fn mutation() -> SemioBrepMutation {
    serde_json::from_str(MUTATION).expect("replace-surface mutation decodes")
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

/// 🔣️ Snapshots and the payload are canonical — `BrepSurface` is internally tagged on `kind` with camelCase VARIANTS but snake_case FIELDS (`half_angle`, `major_radius`, `u_count`).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioBrepSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-surface mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-surface mutation reparses");
    assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the face exists and the new surface genuinely differs, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-surface/swaps-the-faces-plane-for-a-cylinder: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "replacing a surface with a genuinely different one must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `faces.modified`, and its per-face diff must carry `surface` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-surface/swaps-the-faces-plane-for-a-cylinder: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed replace-surface diff decodes");
    
    let faces = decoded.faces.as_ref().expect("replace-surface must write the faces triple");
    assert!(faces.removed.is_empty() && faces.added.is_empty(), "a replace is a per-field modification, never a remove-and-re-add");
    assert_eq!(faces.modified.len(), 1, "exactly one face is modified");
    let face_diff = &faces.modified[0].diff;
    assert!(face_diff.outer_loop.is_none() && face_diff.inner_loops.is_none() && face_diff.orientation.is_none(), "the per-face diff must leave the loop and orientation fields unset");
    assert!(decoded.vertices.is_none() && decoded.edges.is_none() && decoded.loops.is_none() && decoded.shells.is_none() && decoded.solids.is_none(), "no other collection may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioBrepDiff = serde_json::from_str(DIFF).expect("committed replace-surface diff decodes");
    let produced = decoded.apply(&before()).expect("committed replace-surface diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-surface/swaps-the-faces-plane-for-a-cylinder: committed diff did not carry before to after");
}
