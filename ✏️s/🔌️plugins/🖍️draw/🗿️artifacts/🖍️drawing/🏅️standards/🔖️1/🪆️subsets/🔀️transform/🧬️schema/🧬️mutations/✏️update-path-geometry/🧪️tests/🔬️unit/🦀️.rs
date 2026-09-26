//! ✏️ Path geometry changes roundtrip through the semantic mutation and inverse.
use crate::{DrawingSnapshot, DrawingLayerNode, PathSegment};

#[test]
fn canonical_geometry_scenario_matches_diff_apply_and_inverse() {
    use protocol::{Mutation, MutationDiff};
    let before: DrawingSnapshot = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after: DrawingSnapshot = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let mutation: crate::DrawingMutation = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/🦠️mutation/🔣️.json")).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/🔺️diff/🔣️.json")).unwrap();
    let result = mutation.diff(&before);
    assert!(result.messages().is_empty());
    assert_eq!(*result.diff(), serde_json::from_value::<crate::DrawingDiff>(expected).unwrap());
    assert_eq!(result.diff().apply(&before).unwrap(), after);
    let mut restored = after;
    for undo in mutation.inverse(&before) { crate::mutations::apply_drawing_mutation(&mut restored, &undo).unwrap(); }
    assert_eq!(restored, before);
    eprintln!("[DEBUG] canonical path geometry scenario preserves identity and roundtrips through inverse");
}

#[test]
fn path_geometry_mutation_roundtrip_fixture() {
    use protocol::Mutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let before: Vec<PathSegment> = serde_json::from_value(fixture["before"].clone()).unwrap();
    let after: Vec<PathSegment> = serde_json::from_value(fixture["after"].clone()).unwrap();
    let layer = crate::schema::create_drawing_path_layer("Curve", before);
    let id = crate::schema::layer_id(&layer).to_string();
    let document = DrawingSnapshot { layers: vec![layer], ..Default::default() };
    let mutation = super::mutation::update_path_geometry(id, after.clone());
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let decoded: Vec<PathSegment> = dsl::json::from_json_str(&fixture["after"].to_string()).unwrap();
    assert_eq!(decoded, after);
    let inverse = mutation.inverse(&document);
    let mut edited = document.clone();
    crate::mutations::apply_drawing_mutation(&mut edited, &mutation).unwrap();
    let DrawingLayerNode::Path(path) = &edited.layers[0] else { panic!("Expected a path") };
    assert_eq!(path.segments, after);
    for mutation in inverse { crate::mutations::apply_drawing_mutation(&mut edited, &mutation).unwrap(); }
    assert_eq!(edited, document);
}
