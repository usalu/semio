//! 🧪️ `📄set-snapshot` fixture — `slides-the-wall-and-attaches-a-fire-rating-pset`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🧱 The case slides element `w-1` 3.5 m along X and gives it a `Pset_WallCommon` property set.
//! Its `class`, its `Mesh` geometry reference and its `spatialId` containment are unchanged, as are
//! both spatial nodes and the `aggregates` relation — so `SemioModelDiff` must carry an ID-keyed
//! `elements` triple modifying `"w-1"` alone, with `spatial` and `relations` absent.
//!
//! 🪆️ `SemioModelElementDiff::spatial_id` is a tri-state `Option<Option<String>>`; "unparent this
//! element" would be `Some(None)`, which serde writes as bare `null` and reads back as `None`
//! (= unchanged), so no committed fixture may express it. Keeping the containment stable here
//! leaves the slot absent, which is the shape that does round-trip.

use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::SemioModelDiff;
use crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::{apply_semio_model_mutation, SemioModelMutation};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{GeometryRef, SemioModelSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioModelSnapshot {
    serde_json::from_str(BEFORE).expect("before model snapshot decodes")
}
fn expected_after() -> SemioModelSnapshot {
    serde_json::from_str(AFTER).expect("after model snapshot decodes")
}
fn mutation() -> SemioModelMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the building model to exactly the committed `after`: a relocated wall
/// carrying one property set.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_model_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-model/set-snapshot: a genuinely changed model must not raise any message");
    let wall = &snapshot.elements[0];
    assert_eq!(wall.placement.translation.x, 3.5, "semio-model/set-snapshot: the wall must slide along X");
    assert_eq!(wall.psets.len(), 1, "semio-model/set-snapshot: the property set must be attached");
    assert_eq!(wall.spatial_id.as_deref(), Some("b-1"), "semio-model/set-snapshot: sliding a wall must not detach it from its spatial container");
    assert!(matches!(wall.geometry, GeometryRef::Mesh { .. }), "semio-model/set-snapshot: the geometry reference must stay a Mesh handle");
    assert_eq!(snapshot, expected_after(), "semio-model/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must slide the wall back to the
/// origin and strip the property set again.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_model_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_model_mutation(&mut snapshot, step);
    }
    assert!(snapshot.elements[0].psets.is_empty(), "semio-model/set-snapshot: the inverse must remove the attached property set");
    assert_eq!(snapshot, base, "semio-model/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed models and the mutation are already canonical. The model-specific trap
/// pinned here: `GeometryRef` and `ElementClass` are internally tagged on `kind`, and a container
/// `rename_all` renames only the VARIANT — so `GeometryRef::Mesh`'s payload field stays
/// `mesh_id` on the wire while the surrounding `SemioModelElement` is camelCase (`spatialId`).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioModelSnapshot = serde_json::from_str(text).expect("model snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("model snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("model snapshot reparses");
        assert_eq!(reencoded, original, "semio-model/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-model/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the model really moves, so the `mutation.no-op` warning
/// an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-model/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_model_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-model/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-model/set-snapshot: an applied set-snapshot must actually move the model");
}

/// 🔺️ The sparse `SemioModelDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the spatial tree and the relation must stay absent, and the element
/// patch must set only `placement` and the whole-vector `psets`, leaving `class`, `geometry` and
/// the tri-state `spatialId` unset.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced model diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed model diff decodes");
    assert_eq!(produced, committed, "semio-model/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioModelDiff`: one element patched
/// by id, no removals, no additions, and the tri-state containment slot left absent.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioModelDiff = serde_json::from_str(DIFF).expect("committed model diff decodes");
    assert!(decoded.spatial.is_none() && decoded.relations.is_none(), "semio-model/set-snapshot: neither the spatial tree nor the relations may be re-emitted");
    let elements = decoded.elements.as_ref().expect("the committed diff carries an elements triple");
    assert!(elements.removed.is_empty() && elements.added.is_empty() && elements.modified.len() == 1 && elements.modified[0].key == "w-1", "semio-model/set-snapshot: exactly the wall may be patched, addressed by id");
    let patch = &elements.modified[0].diff;
    assert!(patch.class.is_none() && patch.geometry.is_none() && patch.spatial_id.is_none(), "semio-model/set-snapshot: class, geometry and the tri-state spatial_id did not move and must stay absent");
    let reencoded = serde_json::to_value(&decoded).expect("model diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed model diff reparses");
    assert_eq!(reencoded, original, "semio-model/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — placement
/// plus property sets is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioModelDiff = serde_json::from_str(DIFF).expect("committed model diff decodes");
    let produced = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&decoded, &before()).expect("committed model diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-model/set-snapshot: committed diff did not carry before to after");
}
