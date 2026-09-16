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
    let node = render(&spec, &semio_framework_plugin::TreeWindows::unhosted()).expect("steps tree");
    let json = serde_json::to_string(&node).expect("serialize semantic UI fixture");
    assert!(json.contains("Intro"), "step title must appear as a root node label: {json}");
    assert!(json.contains("Name (text)"), "block label+kind must appear as a leaf node label: {json}");
}

#[semio_framework_async_macros::async_test]
async fn render_falls_back_to_the_step_id_when_the_title_is_empty() {
    let step = PlaybookStep { id: "s1".into(), title: String::new(), description: None, blocks: Vec::new() };
    let spec = playbook_snapshot_with_steps("playbook.program", "playbook", "1", None, vec![step]);
    let node = render(&spec, &semio_framework_plugin::TreeWindows::unhosted()).expect("steps tree");
    let json = serde_json::to_string(&node).expect("serialize semantic UI fixture");
    assert!(json.contains("\"s1\""), "an empty step title must fall back to the step id: {json}");
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

/// 🪟️ A recipe with far more blocks in one step than the 32 siblings this window used to refuse outright.
fn oversized_spec(blocks: usize) -> crate::PlaybookSnapshot {
    let step = PlaybookStep { id: "s1".into(), title: "Intro".into(), description: None, blocks: (0..blocks).map(|index| sample_block(&format!("b{index}"), &format!("Field {index}"), "text")).collect() };
    playbook_snapshot_with_steps("playbook.program", "playbook", "1", Some("Recipe".into()), vec![step])
}

/// 🪟️ The window body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(spec: &crate::PlaybookSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(spec, &TreeWindows::for_body(&view, PLAYBOOK_VIEW_BODY_STEPS)).expect("steps tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the steps tree")
}

/// 🪟️ Law (a): a 300-block step renders — the step stamps its full extent and no `+N` appears.
#[test]
fn oversized_step_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_spec(300), Vec::new());
    assert!(json.contains("\"total\":300"), "the step stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("s1/b").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a step the host closed stamps its total and materialises nothing.
#[test]
fn closed_step_stamps_total_and_materialises_no_blocks() {
    let json = window_body(&oversized_spec(300), vec![TreeWindowRequest { body_key: PLAYBOOK_VIEW_BODY_STEPS.into(), node_key: "s1".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed step still stamps its extent: {json}");
    assert!(!json.contains("s1/b"), "a closed step materialises no blocks: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw block id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_spec(300), vec![TreeWindowRequest { body_key: PLAYBOOK_VIEW_BODY_STEPS.into(), node_key: "s1".into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the step reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"s1/b{index}\"")), "block {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"s1/b99\""), "the block before the window stays out: {json}");
    assert!(!json.contains("\"s1/b110\""), "the block after the window stays out: {json}");
}

/// 🪟️ Law (d) for a read-only `TreeWindowKit` surface: no interaction domain, no pick granularity.
#[test]
fn steps_tree_binds_no_interaction_domain() {
    let json = window_body(&oversized_spec(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the steps tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the steps tree stamps no pick granularity: {json}");
}
//#endregion 🪟️WindowLaws
