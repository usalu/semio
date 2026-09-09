use super::*;
use crate::playbook::{PlaybookBlock, PlaybookStep};
use crate::playbook_snapshot_with_steps;

fn sample_block(id: &str, label: &str, kind: &str) -> PlaybookBlock {
    PlaybookBlock {
        id: id.into(),
        label: label.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn definition_restamps_the_tree_window_kit_with_this_windows_own_id_and_body_key() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_VIEW_WINDOW_STEPS);
    assert_eq!(definition.body_key, PLAYBOOK_VIEW_BODY_STEPS);
    assert_eq!(definition.surface_kind, TreeWindowKit::window_kind().surface_kind, "restamping the id/body-key must not change the underlying surface shape");
}

#[semio_framework_async_macros::async_test]
async fn render_nests_every_blocks_label_and_kind_under_its_own_step() {
    let step = PlaybookStep { id: "s1".into(), title: "Intro".into(), description: None, blocks: vec![sample_block("b1", "Name", "text")] };
    let spec = playbook_snapshot_with_steps("playbook.program", "playbook", "1", Some("Recipe".into()), vec![step]);
    let node = render(&spec).expect("steps tree");
    let json = serde_json::to_string(&node).expect("serialize semantic UI fixture");
    assert!(json.contains("Intro"), "step title must appear as a root node label: {json}");
    assert!(json.contains("Name (text)"), "block label+kind must appear as a leaf node label: {json}");
}

#[semio_framework_async_macros::async_test]
async fn render_falls_back_to_the_step_id_when_the_title_is_empty() {
    let step = PlaybookStep { id: "s1".into(), title: String::new(), description: None, blocks: Vec::new() };
    let spec = playbook_snapshot_with_steps("playbook.program", "playbook", "1", None, vec![step]);
    let node = render(&spec).expect("steps tree");
    let json = serde_json::to_string(&node).expect("serialize semantic UI fixture");
    assert!(json.contains("\"s1\""), "an empty step title must fall back to the step id: {json}");
}
