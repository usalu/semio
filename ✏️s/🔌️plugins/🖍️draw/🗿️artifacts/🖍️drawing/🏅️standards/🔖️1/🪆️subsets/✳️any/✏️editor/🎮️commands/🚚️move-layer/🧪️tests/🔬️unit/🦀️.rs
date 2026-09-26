//! 🚚️ Layer drop fixtures preserve identifiers and reject invalid hierarchy edits.
use super::*;
#[test]
fn layer_drop_fixtures() {
    let mut layers = vec![crate::schema::create_drawing_shape_layer_rect("A"), crate::schema::create_drawing_shape_layer_rect("B"), crate::schema::create_drawing_group_layer("G")];
    for (layer, id) in layers.iter_mut().zip(["a.a", "b.b", "g.g"]) { crate::schema::layer_base_mut(layer).id = id.into(); }
    let document = DrawingSnapshot { layers, ..Default::default() };
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let result = plan(&document, &MoveLayer { layer_id: case["source"].as_str().unwrap().into(), target_row_id: case["target"].as_str().unwrap().into(), drop_position: case["position"].as_str().unwrap().into() });
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{case}");
        if let Some(index) = case["index"].as_u64() { let DrawingMutation::ReorderLayer(payload) = result.unwrap() else { panic!("Expected a reorder") }; assert_eq!(payload.index, index as usize); }
    }
}
