//! 🧪️ `rotate-layers` fixture — `🧭️quarter-turn`. The committed quintet is computed by an independent Python implementation
//! (`🧪️w3-t-draw-selection-leaves.py`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`); these laws hold the Rust leaf to it
//! byte for byte and pin the rotate handle's equivalence and the leaf's outcome and label laws.
use crate::mutations::{apply_drawing_mutation, inverse_drawing_mutation, rotate_layers, DrawingMutation};
use crate::DrawingSnapshot;
use protocol::{Mutation, SemanticMutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

/// ▶️ The quarter turn about `(10, 0)` lands exactly on the committed after-document and diff.
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
    for step in inverse_drawing_mutation(&base, &mutation).expect("valid retained mutation inverse fixture") {
        apply_drawing_mutation(&mut snapshot, &step).unwrap();
    }
    assert_eq!(snapshot, base);
}

/// 🎛️ The leaf equals the rotate handle's world matrix applied through the layer's parent: `motion · world`.
#[test]
fn equals_the_rotate_handle_matrix() {
    let bounds = [0.0, 0.0, 120.0, 60.0];
    let handle_motion = crate::schema::geometry::handles::handle_motion(8, bounds, [60.0, -28.0], [150.0, 30.0], false, false).unwrap();
    let crate::schema::geometry::handles::HandleMotion::Rotate { pivot, angle } = handle_motion else { panic!("the rotation handle rotates") };
    assert_eq!(pivot, [60.0, 30.0], "the rotation handle turns about the bounds' centre");
    let motion = handle_motion.matrix();
    let mut snapshot = before();
    apply_drawing_mutation(&mut snapshot, &rotate_layers(vec!["shape-a".into()], pivot[0], pivot[1], angle)).unwrap();
    let actual = crate::schema::drawing_transform_to_matrix(&crate::schema::layer_base(&snapshot.layers[0]).transform);
    let wanted = crate::schema::geometry::multiply(motion, crate::schema::drawing_transform_to_matrix(&crate::schema::layer_base(&before().layers[0]).transform));
    for index in 0..6 {
        assert!((actual[index] - wanted[index]).abs() < 1e-9, "{actual:?} vs {wanted:?}");
    }
}

/// ⚠️ Missing targets are `mutation.target-missing`, a zero angle is `mutation.no-op`, an empty target set an invariant.
#[test]
fn outcome_laws() {
    let base = before();
    let codes = |mutation: DrawingMutation| mutation.diff(&base).messages().iter().map(|message| (format!("{:?}", message.level), message.code.0.clone())).collect::<Vec<_>>();
    assert_eq!(codes(rotate_layers(vec!["ghost".into()], 0.0, 0.0, 1.0)), vec![("Error".into(), "mutation.target-missing".into())]);
    assert_eq!(codes(rotate_layers(vec!["shape-a".into()], 0.0, 0.0, 0.0)), vec![("Warning".into(), "mutation.no-op".into())]);
    assert_eq!(codes(rotate_layers(Vec::new(), 0.0, 0.0, 1.0)), vec![("Fatal".into(), "mutation.invariant".into())]);
}

/// 🗣️ The history row label reads the rotation in degrees, in English and German.
#[test]
fn label_reads_degrees() {
    let label = serde_json::from_str::<DrawingMutation>(MUTATION).unwrap().label();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Rotate 1 layer by 90°");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Ebene um 90° drehen");
}
