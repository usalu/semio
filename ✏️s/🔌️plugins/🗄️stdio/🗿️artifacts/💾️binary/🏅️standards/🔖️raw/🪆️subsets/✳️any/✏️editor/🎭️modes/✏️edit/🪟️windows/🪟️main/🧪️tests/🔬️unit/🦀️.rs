use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_an_editable_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "textEdit"), "editable text window must carry the replace-text catalog action");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_bytes_as_editable_hex() {
    let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert!(scene.buffer.starts_with("deadbeef"));
    assert!(scene.buffer.contains("total bytes: 4"));
    assert!(scene.settings_json.is_none(), "editable window must not stamp readOnly");
}

#[test]
fn render_never_drops_a_tail_that_an_apply_would_replace() {
    let bytes: Vec<u8> = (0..(4096 + 17)).map(|index| (index % 251) as u8).collect();
    let document = BinarySnapshot { bytes: bytes.clone(), ..BinarySnapshot::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    let expected: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    assert!(scene.buffer.starts_with(&expected));
    assert!(scene.buffer.contains("total bytes: 4113"));
    assert!(!scene.buffer.contains("showing first"));
}
