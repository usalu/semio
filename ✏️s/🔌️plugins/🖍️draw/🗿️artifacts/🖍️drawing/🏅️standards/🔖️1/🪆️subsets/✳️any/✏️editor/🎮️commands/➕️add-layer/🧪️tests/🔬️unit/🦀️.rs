//! ➕️ Repeated creation must always address distinct layers.
use super::*;
#[test]
fn repeated_layer_creation_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut document = DrawingSnapshot::default();
    for kind in fixture["kinds"].as_array().unwrap() {
        for _ in 0..fixture["repetitions"].as_u64().unwrap() {
            let layer = build_layer(&document, kind.as_str().unwrap(), None).unwrap();
            assert!(crate::schema::find_drawing_layer(&document, crate::schema::layer_id(&layer)).is_none());
            document.layers.push(layer);
        }
    }
    assert_eq!(document.layers.len(), 30);
}
