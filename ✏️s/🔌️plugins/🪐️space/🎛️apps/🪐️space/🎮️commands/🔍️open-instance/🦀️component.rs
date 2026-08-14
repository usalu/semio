//! 🔍️ 🔍️ S Studio app command — `open-instance`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "open-instance")]
pub struct OpenInstance {
    pub node_id: Option<String>,
}

/// 🕹️ Selection now only informs which node opens, not a `SetSelection` config mutation (the
/// framework owns `graph`'s selection state now — ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn open_with_selection(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, config: &SpaceConfig, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    match payload.node_id.clone().or_else(|| crate::apps::space::primary_selected_node_id(selected, config)) {
        Some(node_id) => match doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id) {
            Some(node) => Emit {
                config_mutations: vec![SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) }, SpaceConfigMutation::SetActiveNode { node_id: Some(node_id.clone()) }],
                effects: vec![HostEffect::OpenPluginInstance { plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), os_instance_id: Some(node.id.clone()) }],
                ..Default::default()
            },
            None => Emit::default(),
        },
        None => Emit::default(),
    }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead); `payload.node_id` (when set) is unaffected — only the "fall back to the
/// live selection" step degrades to empty.
pub fn handle(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(open_with_selection(payload, doc, cfg.snapshot, &[]))
}

pub fn apply(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(open_with_selection(payload, doc, cfg.snapshot, &interaction.selection("graph").ids))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::{apply_config, seed_draw_plugin, studio_emit};
    use crate::apps::space::SpaceCommand;
    use crate::demo_space_projection;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenInstance(OpenInstance { node_id: Some("n1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CloseFocusedInstance(crate::apps::space::commands::close_focused_instance::CloseFocusedInstance {}));
    }

    #[test]
    fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(OpenInstance { node_id: Some(node.id.clone()) })).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "opening an instance is a host effect, not a document operation");
        let opened = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => Some((plugin_id.clone(), app_id.clone(), os_instance_id.clone())),
                _ => None,
            })
            .expect("OpenPluginInstance effect");
        assert_eq!(opened.0, "draw");
        assert_eq!(opened.1, "draw");
        assert_eq!(opened.2.as_deref(), Some(node.id.as_str()));
    }

    #[test]
    fn open_and_close_focused_instance() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.graph.nodes.first().expect("node").id.clone();
        let open_emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(OpenInstance { node_id: Some(node_id.clone()) })).expect("handle");
        assert!(open_emit.config_mutations.contains(&SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) }));
        let config_after_open = apply_config(&config, &open_emit.config_mutations);
        assert_eq!(config_after_open.focused_node_id.as_deref(), Some(node_id.as_str()));
        let close_emit = studio_emit(&projection, &config_after_open, &SpaceCommand::CloseFocusedInstance(crate::apps::space::commands::close_focused_instance::CloseFocusedInstance {})).expect("handle");
        assert_eq!(close_emit.config_mutations, vec![SpaceConfigMutation::SetFocusedNode { node_id: None }]);
    }
}
//#endregion 🧪️Tests
