
use super::*;
use crate::demo_space_projection;
use crate::engine::space::config::SpaceConfig;

/// 🆔️ Adapted from this file's pre-port test (same assertion intent — the label field's action
/// must be `patchAppInstances` — via JSON substrings instead of `UiNode`/`UiControlNode`
/// destructuring, which no longer applies to the ported `BuiltNode` tree shape).
#[semio_framework_async_macros::async_test]
async fn inspector_tree_exposes_the_label_field_action() {
    let projection = demo_space_projection().await;
    let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let config = SpaceConfig::default();
    let node = render(&projection, &ids, semio_framework_plugin::resolve_labels::<SStudioLabels>(&semio_framework_plugin::ViewModel::default())).expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("inspector tree projection");
    assert!(json.contains("s-play-inspector.app-instance.label"), "label field id must reach the tree: {json}");
    assert!(json.contains("patchAppInstances"), "label field action must reach the tree: {json}");
}

#[semio_framework_async_macros::async_test]
async fn empty_selection_renders_the_select_hint_only() {
    let projection = demo_space_projection().await;
    let node = render(&projection, &[], &SStudioLabels::NATIVE_EN).expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("inspector tree projection");
    assert!(json.contains("s-play-inspector.header"), "header section must always render: {json}");
    assert!(!json.contains("s-play-inspector.media-nodes"), "no selection must not render the media-nodes section: {json}");
}
