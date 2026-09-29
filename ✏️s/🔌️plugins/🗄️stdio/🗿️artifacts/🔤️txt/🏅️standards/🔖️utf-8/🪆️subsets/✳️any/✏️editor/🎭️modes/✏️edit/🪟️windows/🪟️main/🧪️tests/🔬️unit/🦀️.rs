use super::*;
use semio_framework_plugin::Component;

/// 🚚️ Decodes the text surface and re-attaches every lane its carrier children hold (`SceneDoc::merge_lane`).
fn merged_scene(node: &BuiltNode) -> semio_framework_ui_scene::TextEditorScene {
    let Component::Surface(props) = &node.component else { panic!("expected a retained text surface") };
    let mut scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("decode text scene");
    for carrier in &node.children {
        let mut payload = String::new();
        let mut frontier = vec![carrier];
        while let Some(child) = frontier.pop() {
            if let Component::Text(text) = &child.component {
                payload.push_str(&text.packed_payload());
            }
            frontier.extend(child.children.iter().rev());
        }
        semio_framework_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), payload);
    }
    scene
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_joins_lines_with_the_line_ending() {
    let document = TxtSnapshot { schema: "stdio.txt".into(), lines: vec!["a".into(), "b".into()], trailing_newline: false, line_ending: Default::default() };
    let node = render(&document, semio_framework_plugin::Locale::En, "revision").expect("render");
    let scene = merged_scene(&node);
    assert_eq!(scene.buffer, "a\nb");
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("editable draft settings")).expect("settings JSON");
    assert_eq!(settings["editAction"], "textEdit");
    assert_eq!(settings["editArgument"], "text");
    assert_eq!(settings["editArguments"]["revision"], "revision");
    assert_eq!(settings["commit"], "explicit");
}
