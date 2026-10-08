//! 🧪️ `drag-layers` fixture — `✋️drags-a-child`. The committed quintet is computed by an independent Python implementation
//! (`🧪️w3-t-draw-selection-leaves.py`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`); these laws hold the Rust leaf to it
//! byte for byte and pin the partial, target-missing, no-op, invariant and label outcomes.
use crate::mutations::{apply_drawing_mutation, drag_layers, inverse_drawing_mutation, DrawingMutation};
use crate::DrawingSnapshot;
use protocol::{Mutation, MutationDiff, SemanticMutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/🎯️outcome/🔣️.json");

fn before() -> DrawingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

fn codes(mutation: &DrawingMutation, base: &DrawingSnapshot) -> Vec<(String, String)> {
    mutation.diff(base).messages().iter().map(|message| (format!("{:?}", message.level), message.code.0.clone())).collect()
}

/// ▶️ The leaf carries `before` to exactly the committed `after`, the root layer and the child of a scaled group both by
/// the same world offset.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_drawing_mutation(&mut snapshot, &serde_json::from_str(MUTATION).expect("mutation decodes")).expect("drag-layers applies");
    assert_eq!(snapshot, serde_json::from_str::<DrawingSnapshot>(AFTER).expect("after decodes"));
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).unwrap()["status"], "applied");
}

/// 🔺️ The produced diff is exactly the committed one, and nothing is reported.
#[test]
fn produces_committed_diff() {
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    let outcome = mutation.diff(&before());
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(serde_json::to_value(outcome.diff()).unwrap(), serde_json::from_str::<serde_json::Value>(DIFF).unwrap());
    let decoded: crate::DrawingDiff = serde_json::from_str(DIFF).unwrap();
    assert_eq!(protocol::apply_diff(&decoded, &before()).unwrap(), serde_json::from_str::<DrawingSnapshot>(AFTER).unwrap(), "the committed diff alone carries before to after");
}

/// ↩️ The inverse restores every moved transform exactly.
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

/// 🔣️ The committed JSON is canonical and the leaf round-trips through its text and binary op codecs.
#[test]
fn committed_json_is_canonical_and_codecs_round_trip() {
    for text in [BEFORE, AFTER] {
        let decoded: DrawingSnapshot = serde_json::from_str(text).unwrap();
        assert_eq!(serde_json::to_value(&decoded).unwrap(), serde_json::from_str::<serde_json::Value>(text).unwrap());
    }
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    assert_eq!(serde_json::to_value(&mutation).unwrap(), serde_json::from_str::<serde_json::Value>(MUTATION).unwrap());
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
}

/// ⚠️ A missing or locked target is skipped with `mutation.partial`, the rest still moves; nothing left is
/// `mutation.target-missing`; a zero offset is `mutation.no-op`; an empty or repeated target set is an invariant.
#[test]
fn outcome_laws() {
    let base = before();
    assert_eq!(codes(&drag_layers(vec!["shape-a".into(), "ghost".into()].into(), 5.0, 0.0), &base), vec![("Warning".into(), "mutation.partial".into())]);
    let mut locked = base.clone();
    crate::schema::layer_base_mut(&mut locked.layers[1]).locked = true;
    assert_eq!(codes(&drag_layers(vec!["shape-a".into(), "shape-b".into()].into(), 5.0, 0.0), &locked), vec![("Warning".into(), "mutation.partial".into())], "a child of a locked group is skipped");
    assert_eq!(codes(&drag_layers(vec!["shape-b".into()].into(), 5.0, 0.0), &locked), vec![("Error".into(), "mutation.target-missing".into())]);
    assert_eq!(codes(&drag_layers(vec!["ghost".into()].into(), 5.0, 0.0), &base), vec![("Error".into(), "mutation.target-missing".into())]);
    assert_eq!(codes(&drag_layers(vec!["shape-a".into()].into(), 0.0, 0.0), &base), vec![("Warning".into(), "mutation.no-op".into())]);
    assert_eq!(codes(&drag_layers(Vec::new().into(), 5.0, 0.0), &base), vec![("Fatal".into(), "mutation.invariant".into())]);
    assert_eq!(codes(&drag_layers(vec!["shape-a".into(), "shape-a".into()].into(), 5.0, 0.0), &base), vec![("Fatal".into(), "mutation.invariant".into())]);
}

/// 🗂️ A child whose addressed group moves rides along exactly once.
#[test]
fn a_child_of_a_dragged_group_moves_once() {
    let base = before();
    let mut snapshot = base.clone();
    apply_drawing_mutation(&mut snapshot, &drag_layers(vec!["group-a".into(), "shape-b".into()].into(), 20.0, -10.0)).unwrap();
    let (crate::DrawingLayerNode::Group(moved), crate::DrawingLayerNode::Group(original)) = (&snapshot.layers[1], &base.layers[1]) else { panic!("group") };
    assert_eq!((moved.base.transform.x, moved.base.transform.y), (120.0, 40.0));
    assert_eq!(moved.children, original.children, "the child keeps its local transform and moves with its group");
}

/// 🗣️ The history row label reads the drag in English and German.
#[test]
fn label_reads_the_drag() {
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    let label = mutation.label();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 2 layers by (20, -10)");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "2 Ebenen um (20; -10) ziehen");
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-document.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    let mutation: DrawingMutation = serde_json::from_str(MUTATION).unwrap();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before()).await;
}
