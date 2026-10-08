use super::*;
use semio_framework_plugin::{ActionBinding, Component};

fn selection_fixture() -> (TiffSnapshot, TiffEditorConfig, Vec<u8>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️ifd-selection/🔣️.json")).expect("neutral IFD selection fixture");
    let mut snapshot = crate::schema::blank_tiff_snapshot();
    snapshot.ifds.clear();
    for page in fixture["pages"].as_array().expect("fixture pages") {
        let mut ifd = crate::schema::blank_tiff_snapshot().ifds.remove(0);
        ifd.blocks[0].samples = page["rgb"].as_array().expect("fixture RGB").iter().map(|value| crate::schema::snapshot::TiffWord64::from_word(value.as_u64().expect("RGB component"))).collect();
        snapshot.ifds.push(ifd);
    }
    let config = TiffEditorConfig { selected_ifd: fixture["selectedIfd"].as_u64().expect("fixture selected IFD") as usize };
    let expected = fixture["expectedSelectedRgba"].as_array().expect("fixture selected RGBA").iter().map(|value| value.as_u64().expect("RGBA byte") as u8).collect();
    (snapshot, config, expected)
}

fn node_by_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| node_by_key(child, key)))
}

fn binding(node: &BuiltNode, action: &str) -> Option<ActionBinding> {
    node.bindings.iter().find(|binding| binding.action.name.as_str() == action).and_then(|binding| binding.credited_clone()).or_else(|| node.children.iter().find_map(|child| binding(child, action)))
}

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = TiffSnapshot::default();
    let _node = render(&document, &TiffEditorConfig::default(), Locale::En);
}

#[semio_framework_async_macros::async_test]
async fn selected_ifd_renders_bilingual_accessible_controls_and_the_selected_page() {
    let (snapshot, config, expected) = selection_fixture();
    let english = render(&snapshot, &config, Locale::En).expect("selected TIFF page renders");
    let previous = node_by_key(&english, "tiff-image-page-previous").expect("previous page button");
    let Component::Button(previous) = &previous.component else { panic!("page navigation uses a native button") };
    assert_eq!(previous.label.0.as_str(), "Previous image page");
    let binding = binding(&english, SELECT_IFD_ACTION_ID).expect("page control action binding");
    let Some(UiValue::Map(arguments)) = binding.args else { panic!("page control carries an address") };
    assert_eq!(arguments.iter().find(|(key, _)| key.as_str() == "ifdIndex").map(|(_, value)| value), Some(UiValue::Number(0.0)));
    let status = node_by_key(&english, "tiff-image-page-status").expect("English page status");
    let Component::Text(status) = &status.component else { panic!("page status is text") };
    assert_eq!(status.value.0.as_str(), "Image page 2 of 2");
    let selected = node_by_key(&english, semio_framework_plugin::app::IMAGE_WINDOW_CONTENT_NODE_KEY).expect("selected page image");
    let Component::Image(selected) = &selected.component else { panic!("selected page projection is an image") };
    assert_eq!(selected.alt.as_ref().map(|label| label.0.as_str()), Some("1x1"));
    let encoded = selected.src.as_str().strip_prefix("data:image/png;base64,").expect("rendered selected page data URI");
    let png = semio_framework_value::base64_standard_decode(encoded).expect("rendered selected page base64");
    let decoded = image::load_from_memory_with_format(&png, image::ImageFormat::Png).expect("image-rs independently decodes rendered selected preview").to_rgba8();
    assert_eq!(decoded.as_raw(), &expected);
    let german = render(&snapshot, &config, Locale::De).expect("German TIFF selector renders");
    let Component::Button(previous) = &node_by_key(&german, "tiff-image-page-previous").expect("German previous page button").component else { panic!("page navigation uses a native button") };
    assert_eq!(previous.label.0.as_str(), "Vorherige Bildseite");
}

#[semio_framework_async_macros::async_test]
async fn stale_selection_safely_resets_to_ifd_zero_without_changing_the_document() {
    let (snapshot, _, _) = selection_fixture();
    let before = snapshot.clone();
    let stale = TiffEditorConfig { selected_ifd: usize::MAX };
    assert_eq!(selected_ifd(&stale, &snapshot), Some(0));
    let root = render(&snapshot, &stale, Locale::En).expect("stale selection clamps");
    assert!(node_by_key(&root, "tiff-image-page-next").is_some());
    assert_eq!(snapshot, before);
    assert_eq!(selected_ifd(&TiffEditorConfig::default(), &snapshot), Some(0), "a reopened editor starts on the first page");
    assert_eq!(selected_ifd(&TiffEditorConfig::default(), &TiffSnapshot::default()), None);
}
