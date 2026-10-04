//! 🧪️ `drag-path-points` fixture — `📍️drags`. The committed quintet is computed by an independent Python
//! implementation (`🧪️w3-t-draw-selection-leaves.py`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`); these laws hold the
//! Rust leaf to it byte for byte — the world offset mapped into the scaled path's axes, each anchor carrying its tangent.
use crate::mutations::{apply_drawing_mutation, drag_path_points, inverse_drawing_mutation, DrawingMutation, DrawingPathPointTarget};
use crate::schema::geometry::editing::PathPoint;
use crate::DrawingSnapshot;
use protocol::{Mutation, SemanticMutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

fn target(layer_id: &str, index: usize, point: PathPoint) -> DrawingPathPointTarget {
    DrawingPathPointTarget { layer_id: layer_id.into(), index, point }
}

/// ▶️ Both anchors and their attached tangents move by the committed local offset.
#[test]
fn applies_to_committed_after_and_diff() {
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    let outcome = mutation.diff(&before());
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(serde_json::to_value(outcome.diff()).unwrap(), serde_json::from_str::<serde_json::Value>(DIFF).unwrap());
    let mut snapshot = before();
    apply_drawing_mutation(&mut snapshot, &mutation).unwrap();
    assert_eq!(snapshot, serde_json::from_str::<DrawingSnapshot>(AFTER).unwrap());
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).unwrap()["status"], "applied");
    assert_eq!(serde_json::to_value(&mutation).unwrap(), serde_json::from_str::<serde_json::Value>(MUTATION).unwrap());
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
}

/// ↩️ The inverse restores the base geometry exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).unwrap();
    for step in inverse_drawing_mutation(&base, &mutation).expect("valid retained mutation inverse fixture") {
        apply_drawing_mutation(&mut snapshot, &step).unwrap();
    }
    assert_eq!(snapshot, base);
}

/// ⚠️ A point beyond the path is `mutation.target-missing`, a locked path too, a zero offset is `mutation.no-op`, an empty
/// or repeated point set is an invariant.
#[test]
fn outcome_laws() {
    let base = before();
    let codes = |mutation: DrawingMutation, base: &DrawingSnapshot| mutation.diff(base).messages().iter().map(|message| (format!("{:?}", message.level), message.code.0.clone())).collect::<Vec<_>>();
    assert_eq!(codes(drag_path_points(vec![target("path-a", 9, PathPoint::Anchor)], 1.0, 0.0), &base), vec![("Error".into(), "mutation.target-missing".into())]);
    assert_eq!(codes(drag_path_points(vec![target("path-a", 1, PathPoint::Anchor), target("ghost", 0, PathPoint::Anchor)], 1.0, 0.0), &base), vec![("Warning".into(), "mutation.partial".into())]);
    let mut locked = base.clone();
    crate::schema::layer_base_mut(&mut locked.layers[0]).locked = true;
    assert_eq!(codes(drag_path_points(vec![target("path-a", 1, PathPoint::Anchor)], 1.0, 0.0), &locked), vec![("Error".into(), "mutation.target-missing".into())]);
    assert_eq!(codes(drag_path_points(vec![target("path-a", 1, PathPoint::Anchor)], 0.0, 0.0), &base), vec![("Warning".into(), "mutation.no-op".into())]);
    assert_eq!(codes(drag_path_points(Vec::new(), 1.0, 0.0), &base), vec![("Fatal".into(), "mutation.invariant".into())]);
    assert_eq!(codes(drag_path_points(vec![target("path-a", 1, PathPoint::Anchor), target("path-a", 1, PathPoint::Anchor)], 1.0, 0.0), &base), vec![("Fatal".into(), "mutation.invariant".into())]);
}

/// 🗣️ The history row label reads the drag, in English and German.
#[test]
fn label_reads_the_drag() {
    let label = serde_json::from_str::<DrawingMutation>(MUTATION).unwrap().label();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 2 path points by (20, 10)");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "2 Pfadpunkte um (20; 10) ziehen");
}
