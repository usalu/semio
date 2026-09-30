//! 🧪️ `scale-layers` fixture — `📐️doubles-a-rect`. The committed quintet is computed by an independent Python implementation
//! (`🧪️w3-t-draw-selection-leaves.py`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`); these laws hold the Rust leaf to it
//! byte for byte and pin the resize handle's equivalence and the leaf's outcome and label laws.
use crate::mutations::{apply_drawing_mutation, inverse_drawing_mutation, scale_layers, DrawingMutation};
use crate::DrawingSnapshot;
use protocol::{Mutation, SemanticMutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles-a-rect/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles-a-rect/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles-a-rect/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles-a-rect/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles-a-rect/🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

/// ▶️ Scaling by `(2, 0.5)` about the rect's own origin lands exactly on the committed after-document and diff.
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

/// ↩️ The inverse restores the base transform exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &mutation).unwrap();
    for step in inverse_drawing_mutation(&base, &mutation) {
        apply_drawing_mutation(&mut snapshot, &step).unwrap();
    }
    assert_eq!(snapshot, base);
}

/// 🎛️ The leaf equals a resize handle's world matrix applied through the layer's parent, the constrained and centered
/// variants included.
#[test]
fn equals_the_resize_handle_matrix() {
    let bounds = [10.0, 20.0, 120.0, 60.0];
    for (handle, constrained, centered) in [(4, false, false), (0, false, false), (4, true, false), (4, false, true), (5, true, true)] {
        let handle_motion = crate::schema::geometry::handles::handle_motion(handle, bounds, [130.0, 80.0], [190.0, 60.0], constrained, centered).unwrap();
        let crate::schema::geometry::handles::HandleMotion::Scale { pivot, scale } = handle_motion else { panic!("a resize handle scales") };
        let motion = handle_motion.matrix();
        let mut snapshot = before();
        apply_drawing_mutation(&mut snapshot, &scale_layers(vec!["shape-a".into()], pivot[0], pivot[1], scale[0], scale[1])).unwrap();
        let actual = crate::schema::drawing_transform_to_matrix(&crate::schema::layer_base(&snapshot.layers[0]).transform);
        let wanted = crate::schema::geometry::multiply(motion, crate::schema::drawing_transform_to_matrix(&crate::schema::layer_base(&before().layers[0]).transform));
        for index in 0..6 {
            assert!((actual[index] - wanted[index]).abs() < 1e-9, "handle {handle}: {actual:?} vs {wanted:?}");
        }
    }
}

/// ⚠️ A zero factor is an invariant, the identity is `mutation.no-op`, missing targets `mutation.target-missing`.
#[test]
fn outcome_laws() {
    let base = before();
    let codes = |mutation: DrawingMutation| mutation.diff(&base).messages().iter().map(|message| (format!("{:?}", message.level), message.code.0.clone())).collect::<Vec<_>>();
    assert_eq!(codes(scale_layers(vec!["shape-a".into()], 0.0, 0.0, 0.0, 1.0)), vec![("Fatal".into(), "mutation.invariant".into())]);
    assert_eq!(codes(scale_layers(vec!["shape-a".into()], 0.0, 0.0, 1.0, 1.0)), vec![("Warning".into(), "mutation.no-op".into())]);
    assert_eq!(codes(scale_layers(vec!["ghost".into()], 0.0, 0.0, 2.0, 2.0)), vec![("Error".into(), "mutation.target-missing".into())]);
}

/// 🗣️ The history row label reads both factors, in English and German.
#[test]
fn label_reads_the_factors() {
    let label = serde_json::from_str::<DrawingMutation>(MUTATION).unwrap().label();
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Scale 1 layer by (2, 0.5)");
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::De), "1 Ebene um (2; 0,5) skalieren");
}
