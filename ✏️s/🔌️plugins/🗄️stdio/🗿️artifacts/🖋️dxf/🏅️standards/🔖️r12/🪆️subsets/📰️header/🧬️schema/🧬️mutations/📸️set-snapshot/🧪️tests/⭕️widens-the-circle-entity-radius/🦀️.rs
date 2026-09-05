//! 🧪️ `📸️set-snapshot` fixture — `⭕️widens-the-circle-entity-radius`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ⭕️ The case grows the group-code-40 radius of the second `ENTITIES` record from 2.5 to 4.0,
//! leaving its centre and layer, the neighbouring `LINE`, the `$INSBASE` header var and the
//! `LAYER` table alone — so `DxfDiff::between` must emit an index-keyed `entities.modified[1]`
//! carrying a `DxfEntityDiff::Circle` with `radius` as its ONLY set field, never the `Replace`
//! fallback that a kind change would produce.

use crate::artifacts::dxf::schema::diff::{DxfDiff, DxfEntityDiff};
use crate::artifacts::dxf::schema::mutations::{apply_dxf_mutation, DxfMutation};
use crate::artifacts::dxf::schema::snapshot::{DxfEntity, DxfSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DxfSnapshot {
    serde_json::from_str(BEFORE).expect("before DXF snapshot decodes")
}
fn expected_after() -> DxfSnapshot {
    serde_json::from_str(AFTER).expect("after DXF snapshot decodes")
}
fn mutation() -> DxfMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the R12 drawing to exactly the committed `after`: the circle is wider,
/// everything else is where it was.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_dxf_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "dxf/set-snapshot: a genuinely changed drawing must not raise any message");
    let DxfEntity::Circle { radius, center, layer, .. } = &snapshot.entities[1] else {
        panic!("dxf/set-snapshot: entity 1 must still be a CIRCLE — a set-snapshot that flips its kind is a different fixture");
    };
    assert_eq!(*radius, 4.0, "dxf/set-snapshot: the circle radius must widen");
    assert_eq!((*center, layer.as_str()), ([5.0, 5.0, 0.0], "0"), "dxf/set-snapshot: widening must not move the circle or restack its layer");
    assert_eq!(snapshot, expected_after(), "dxf/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must shrink the circle back to
/// 2.5 and leave the LINE, the header var and the LAYER table untouched throughout.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <DxfMutation as protocol::Mutation<DxfSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_dxf_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_dxf_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "dxf/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed DXF snapshots and the mutation are already canonical. Two DXF-specific traps
/// are pinned: `DxfEntity` is EXTERNALLY tagged (`{"circle": {…}}`, no `kind` key) and its
/// container `rename_all` renames only the variant, so a variant field such as `start_angle` or
/// `block_name` would stay snake_case; and every entity's empty `unknown_group_codes` retention
/// vector is skipped entirely rather than written as `[]`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DxfSnapshot = serde_json::from_str(text).expect("DXF snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("DXF snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("DXF snapshot reparses");
        assert_eq!(reencoded, original, "dxf/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "dxf/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the drawing really moves, so no diagnostic is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "dxf/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_dxf_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "dxf/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "dxf/set-snapshot: an applied set-snapshot must actually move the drawing");
}

/// 🔺️ The sparse `DxfDiff` this mutation produces is exactly the committed diff — the load-bearing
/// assertion: `header_vars`, `tables` and `blocks` must stay absent, entity 0 must not appear in
/// `entities.modified`, and the circle patch must carry `radius` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <DxfMutation as protocol::Mutation<DxfSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced DXF diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed DXF diff decodes");
    assert_eq!(produced, committed, "dxf/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `DxfDiff` as a kind-preserving
/// `Circle` patch. `DxfDiff` has no `other_tables` slot at all, so a fixture must never place a
/// change there.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: DxfDiff = serde_json::from_str(DIFF).expect("committed DXF diff decodes");
    assert!(decoded.header_vars.is_none() && decoded.tables.is_none() && decoded.blocks.is_none(), "dxf/set-snapshot: nothing but the entities triple may be touched");
    let entities = decoded.entities.as_ref().expect("the committed diff carries an entities triple");
    assert!(entities.removed.is_empty() && entities.added.is_empty() && entities.modified.len() == 1, "dxf/set-snapshot: the circle must be patched in place, never removed and re-added");
    let DxfEntityDiff::Circle(circle) = &entities.modified[0].diff else {
        panic!("dxf/set-snapshot: the delta must be a kind-preserving Circle patch, never a Replace");
    };
    assert!(circle.center.is_none() && circle.layer.is_none() && circle.radius == Some(4.0), "dxf/set-snapshot: only the radius field may be set on the circle patch");
    let reencoded = serde_json::to_value(&decoded).expect("DXF diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed DXF diff reparses");
    assert_eq!(reencoded, original, "dxf/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the single
/// radius field is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: DxfDiff = serde_json::from_str(DIFF).expect("committed DXF diff decodes");
    let produced = <DxfDiff as protocol::MutationDiff<DxfSnapshot>>::apply(&decoded, &before()).expect("committed DXF diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "dxf/set-snapshot: committed diff did not carry before to after");
}
