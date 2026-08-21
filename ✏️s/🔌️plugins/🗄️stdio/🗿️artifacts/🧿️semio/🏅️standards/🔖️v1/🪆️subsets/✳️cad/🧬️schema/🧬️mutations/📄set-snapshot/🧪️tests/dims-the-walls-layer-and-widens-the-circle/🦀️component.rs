//! 🧪️ `📄set-snapshot` fixture — `dims-the-walls-layer-and-widens-the-circle`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🗂️ The case recolours and hides the `WALLS` layer and, independently, widens the circle held by
//! entity handle `h-1`. Layer `0`, the `DOOR` block and entity `h-2` are unchanged — so
//! `SemioCadDiff` must fill its name-keyed `layers` slot and its handle-keyed `entities` slot and
//! leave `blocks` absent. `CadEntityRecordDiff` treats the geometry as a WHOLE VALUE, so the
//! circle arrives re-stated in full rather than as a `radius`-only sub-patch; that is the artifact's
//! own choice and is what this fixture pins.

use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::SemioCadDiff;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::{apply_semio_cad_mutation, SemioCadMutation};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadEntity, SemioCadSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioCadSnapshot {
    serde_json::from_str(BEFORE).expect("before CAD snapshot decodes")
}
fn expected_after() -> SemioCadSnapshot {
    serde_json::from_str(AFTER).expect("after CAD snapshot decodes")
}
fn mutation() -> SemioCadMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the drawing to exactly the committed `after`: a hidden green `WALLS`
/// layer and a wider circle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_cad_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-cad/set-snapshot: a genuinely changed drawing must not raise any message");
    assert!(!snapshot.layers[1].visible, "semio-cad/set-snapshot: the WALLS layer must be hidden");
    assert_eq!(snapshot.layers[1].name, "WALLS", "semio-cad/set-snapshot: a layer's name is its identity and is never rewritten by a patch");
    let CadEntity::Circle { radius, .. } = &snapshot.entities[0].entity else {
        panic!("semio-cad/set-snapshot: entity h-1 must still be a circle — a set-snapshot that changes its kind is a different fixture");
    };
    assert_eq!(*radius, 4.0, "semio-cad/set-snapshot: the circle must widen");
    assert_eq!(snapshot, expected_after(), "semio-cad/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must reveal the `WALLS` layer
/// again and shrink the circle back to 2.5.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_cad_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_cad_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-cad/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed drawings and the mutation are already canonical: `CadEntity` is internally
/// tagged on `kind`, and its container `rename_all` renames only the VARIANT — so a variant field
/// like `start_angle` or `block_name` stays snake_case even though `CadLayer.color_index` is
/// written `colorIndex`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioCadSnapshot = serde_json::from_str(text).expect("CAD snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("CAD snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("CAD snapshot reparses");
        assert_eq!(reencoded, original, "semio-cad/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-cad/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the drawing really moves, so the `mutation.no-op`
/// warning an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-cad/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_cad_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-cad/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-cad/set-snapshot: an applied set-snapshot must actually move the drawing");
}

/// 🔺️ The sparse `SemioCadDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: layer `0`, the `DOOR` block and entity `h-2` must all be absent, the
/// `WALLS` patch must set only `colorIndex`/`visible` (never `lineType`), and the `h-1` patch must
/// set only `entity` (never `layer`).
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced CAD diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed CAD diff decodes");
    assert_eq!(produced, committed, "semio-cad/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioCadDiff`: one layer patched by
/// name, one entity patched by handle, no removals and no additions.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioCadDiff = serde_json::from_str(DIFF).expect("committed CAD diff decodes");
    assert!(decoded.blocks.is_none(), "semio-cad/set-snapshot: the block table must stay untouched");
    let layers = decoded.layers.as_ref().expect("the committed diff carries a layers triple");
    assert!(layers.removed.is_empty() && layers.added.is_empty() && layers.modified.len() == 1 && layers.modified[0].key == "WALLS", "semio-cad/set-snapshot: exactly the WALLS layer may be patched, addressed by name");
    assert!(layers.modified[0].diff.line_type.is_none(), "semio-cad/set-snapshot: the layer's line type did not move and must stay absent");
    let entities = decoded.entities.as_ref().expect("the committed diff carries an entities triple");
    assert!(entities.removed.is_empty() && entities.added.is_empty() && entities.modified.len() == 1 && entities.modified[0].key == "h-1", "semio-cad/set-snapshot: exactly entity h-1 may be patched, addressed by handle");
    assert!(entities.modified[0].diff.layer.is_none(), "semio-cad/set-snapshot: widening a circle must not restate its owning layer");
    let reencoded = serde_json::to_value(&decoded).expect("CAD diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed CAD diff reparses");
    assert_eq!(reencoded, original, "semio-cad/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the layer
/// and entity patches together are a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioCadDiff = serde_json::from_str(DIFF).expect("committed CAD diff decodes");
    let produced = <SemioCadDiff as protocol::MutationDiff<SemioCadSnapshot>>::apply(&decoded, &before()).expect("committed CAD diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-cad/set-snapshot: committed diff did not carry before to after");
}
