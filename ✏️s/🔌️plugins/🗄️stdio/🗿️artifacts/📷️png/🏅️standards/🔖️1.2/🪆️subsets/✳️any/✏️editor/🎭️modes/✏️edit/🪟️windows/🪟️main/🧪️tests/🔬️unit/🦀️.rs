use super::*;
use semio_framework_plugin::Component;

fn node_by_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| node_by_key(child, key)))
}

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = PngSnapshot::default();
    let _node = render(&document, semio_framework_ui_locale::Locale::En);
}

#[semio_framework_async_macros::async_test]
async fn render_identifies_native_precision_and_interlace_in_both_locales() {
    let source = include_bytes!("../../../../../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png");
    let snapshot = crate::io::decode_png(source).unwrap();
    let english = render(&snapshot, Locale::En).unwrap();
    let Component::Text(status) = &node_by_key(&english, "png-native-profile-control").expect("native profile control").component else { panic!("native profile control is accessible text") };
    assert_eq!(status.value.0.as_str(), "Native paint: grayscale, 16-bit samples, valid samples 0–65535");
    let german = render(&snapshot, Locale::De).unwrap();
    let Component::Text(status) = &node_by_key(&german, "png-native-profile-control").expect("German native profile control").component else { panic!("native profile control is accessible text") };
    assert_eq!(status.value.0.as_str(), "Natives Malen: Graustufe, 16-Bit-Abtastwerte, gültige Abtastwerte 0–65535");

    let adam7 = crate::io::decode_png(include_bytes!("../../../../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png")).unwrap();
    let english = render(&adam7, Locale::En).unwrap();
    let Component::Text(status) = &node_by_key(&english, "png-native-profile-control").expect("Adam7 profile control").component else { panic!("native profile control is accessible text") };
    assert!(status.value.0.as_str().contains("Adam7 preserved"));

    let indexed = crate::io::decode_png(include_bytes!("../../../../../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png")).unwrap();
    let english = render(&indexed, Locale::En).unwrap();
    let Component::Text(status) = &node_by_key(&english, "png-native-profile-control").expect("indexed profile control").component else { panic!("native profile control is accessible text") };
    assert!(status.value.0.as_str().contains("valid indices 0–3"));
}
