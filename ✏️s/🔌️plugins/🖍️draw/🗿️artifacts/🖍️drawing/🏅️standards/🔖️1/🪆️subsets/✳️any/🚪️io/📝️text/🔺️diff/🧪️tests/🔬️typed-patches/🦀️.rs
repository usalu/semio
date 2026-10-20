use crate::DrawingLayerPatch;
use crate::diff_set_fill;
use crate::diff_set_stroke;

#[test]
fn typed_patch_preserves_explicit_clear_with_json_oracle() {
    let fixture = include_str!("../../../../../🧪️tests/🧫️fixtures/🔺️diff/🩹️typed-clear/🔣️.json");
    let decoded: DrawingLayerPatch = semio_framework_pack_json::from_json_str(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let oracle: DrawingLayerPatch = serde_json::from_str(fixture).unwrap();
    assert_eq!(decoded, oracle);
    assert_eq!(decoded.fill.as_ref().unwrap().value, None);
    assert_eq!(decoded.stroke.as_ref().unwrap().value, None);
    assert_eq!(decoded.transform, None);
    let encoded = semio_framework_pack_json::to_json_string(&decoded);
    let round_trip: DrawingLayerPatch = semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(round_trip, decoded);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(), serde_json::to_value(&oracle).unwrap());
    assert_ne!(decoded, DrawingLayerPatch::default());
}

#[test]
fn semantic_constructors_preserve_clear_without_encoding() {
    let fill = diff_set_fill("layer", &None);
    let stroke = diff_set_stroke("layer", &None);
    assert_eq!(fill.layers.unwrap().modified[0].patch.fill.as_ref().unwrap().value, None);
    assert_eq!(stroke.layers.unwrap().modified[0].patch.stroke.as_ref().unwrap().value, None);
}
