
use super::*;
use crate::demo_space_projection;
use crate::engine::space::SpaceCommand;
use crate::engine::space::unit_tests::context::{apply_config, seed_draw_plugin, studio_emit};

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenInstance(OpenInstance { node_id: Some("n1".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CloseFocusedInstance(crate::engine::space::commands::close_focused_instance::CloseFocusedInstance {}));
}

#[semio_framework_async_macros::async_test]
async fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
    seed_draw_plugin().await;
    let projection = demo_space_projection().await;
    let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
    let config = SpaceConfig::default();
    let emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(OpenInstance { node_id: Some(node.id.clone()) })).await.expect("handle");
    assert!(emit.artifact_mutations.is_empty(), "opening an instance is a host effect, not a document operation");
    let opened = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => Some((plugin_id.clone(), app_id.clone(), os_instance_id.clone())),
            _ => None,
        })
        .expect("OpenPluginInstance effect");
    assert_eq!(opened.0, "draw");
    assert_eq!(opened.1, "draw");
    assert_eq!(opened.2.as_deref(), Some(node.id.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn open_and_close_focused_instance() {
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node_id = projection.graph.nodes.first().expect("node").id.clone();
    let open_emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(OpenInstance { node_id: Some(node_id.clone()) })).await.expect("handle");
    assert!(open_emit.config_mutations.contains(&SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) }));
    let config_after_open = apply_config(&config, &open_emit.config_mutations).await;
    assert_eq!(config_after_open.focused_node_id.as_deref(), Some(node_id.as_str()));
    let close_emit = studio_emit(&projection, &config_after_open, &SpaceCommand::CloseFocusedInstance(crate::engine::space::commands::close_focused_instance::CloseFocusedInstance {})).await.expect("handle");
    assert_eq!(close_emit.config_mutations, vec![SpaceConfigMutation::SetFocusedNode { node_id: None }]);
}
