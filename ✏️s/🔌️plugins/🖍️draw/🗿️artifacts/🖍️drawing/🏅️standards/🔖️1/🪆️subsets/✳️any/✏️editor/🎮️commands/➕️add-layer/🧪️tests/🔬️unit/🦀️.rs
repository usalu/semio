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

#[test]
fn created_layers_have_the_shared_visible_appearance() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎨️appearance/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let layer=build_layer(&DrawingSnapshot::default(),case["kind"].as_str().unwrap(),None).unwrap();
        let expected:crate::DrawingAttributes=serde_json::from_value(case["attributes"].clone()).unwrap();
        assert_eq!(crate::schema::layer_base(&layer).attributes,expected,"{}",case["kind"]);
    }
}

#[test]
fn creation_keeps_explicit_paint() {
    let mut layer=crate::schema::create_drawing_shape_layer_rect("Styled");
    crate::schema::layer_base_mut(&mut layer).attributes.fill=Some(crate::FillStyle::Solid {color:[1.0,0.0,0.0,0.3]});
    let before=layer.clone();
    initialize_appearance(&mut layer);
    assert_eq!(layer,before);
}
