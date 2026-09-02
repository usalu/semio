//! 🔍️ S Studio app — inspector panel: selected node position (Transform-ish section) and
//! identity/parameter-binding facets (Properties-ish section), both driven off the `graph` interaction
//! domain's live selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — passed in by
//! the caller, since `ArtifactApp::render` carries no `InteractionView` (a discovered framework gap).
//!
//! 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (26/08/20) port: rebuilt on the `semio_framework_ui_
//! contract` builder DSL (`BuiltNode`), replacing the old `ui_wgpu::wgpu::UiNode` struct literals —
//! same treatment, same helper shapes, as the sibling `📌️panels/🔢️parameters/🦀️.rs` port. The old
//! per-file `s_play_action` (returning the legacy `ActionDescriptor`) is gone — `crate::engine::space::
//! s_play_action` already returns the contract's own `(ActionId, Option<UiValue>)` pair.

use crate::engine::space::engine::{os_parameter_types_compatible_shim, parameter_entity_id, workflow_parameter_to_os};
use crate::engine::space::terminology::SStudioLabels;
use crate::engine::space::{ui_value_list, ui_value_map, ui_value_text, S_PLAY_INSPECTOR_TAB_ID};
use semio_framework_os::{os_app_registration, os_parameter_value, WorkflowNode, WorkflowParameter, WorkflowSnapshot};
use semio_framework_plugin::{
    ui_inspector_all_equal, ActionId, Buildable, HasBase, HasChildren, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, UiFixedList, UiText, UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_ui_contract::{BuiltNode, InputKind, Label, Trigger};

//#region 🔖️Manifest
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(S_PLAY_INSPECTOR_TAB_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(crate::engine::space::S_PLAY_INSPECTOR_BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
fn fixed_text(value: &str, code: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new(code, "fixed inspector text admission failed"))
}

fn fixed_label(value: &str, code: &'static str) -> UiAssemblyResult<Label> {
    Label::try_from(value).map_err(|_| PluginAssemblyError::new(code, "fixed inspector label admission failed"))
}

fn plain_text(value: &str, code: &'static str) -> UiAssemblyResult<BuiltNode> {
    semio_framework_ui_contract::text(fixed_label(value, code)?).try_build().map_err(|_| PluginAssemblyError::new(code, "inspector text node admission failed"))
}

/// 🎬️ Binds `trigger` to the `(ActionId, Option<UiValue>)` pair `crate::engine::space::s_play_action`
/// returns — generic over any `HasBase` builder, mirrors `📌️panels/🔢️parameters/🦀️.rs`'s helper.
fn bind_action<B: HasBase>(builder: B, trigger: Trigger, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<B> {
    match action.1 {
        Some(args) => builder.try_on_with(trigger, action.0, args),
        None => builder.try_on(trigger, action.0),
    }
    .map_err(|_| PluginAssemblyError::new("ui.inspector.action", "action binding admission failed"))
}

fn node_ids_arg(ids: &[String]) -> UiAssemblyResult<UiValue> {
    let mut values = Vec::with_capacity(ids.len());
    for id in ids {
        values.push(ui_value_text(id.as_str())?);
    }
    ui_value_list(values)
}

/// 🌉️ `⚙️engine`'s `parameter_entity_id`/`os_parameter_types_compatible_shim`/`workflow_parameter_to_os`
/// are async fns (kernel-shape convention); this whole render tree stays U1-sync (the contract
/// builder's own sync-only ruling, `📌️important.md`), bridged the same way `🎮️commands/🔍️open-instance/
/// 🦀️.rs`'s `handle` bridges its own callee.
fn entity_id(parameter: &WorkflowParameter) -> String {
    crate::engine::space::engine::resolve_future(parameter_entity_id(parameter)).to_string()
}

fn field_node(label: &str, code: &'static str, id: String, child: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let mut children = UiFixedList::<BuiltNode>::default();
    children.try_push(child).map_err(|_| PluginAssemblyError::new(code, "inspector field child admission failed"))?;
    semio_framework_ui_contract::field(fixed_label(label, code)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new(code, "inspector field id admission failed"))?
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new(code, "inspector field child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new(code, "inspector field admission failed"))
}

pub fn render(projection: &WorkflowSnapshot, selected_node_ids: &[String], term_labels: &SStudioLabels) -> UiAssemblyResult<BuiltNode> {
    let mut sections = UiFixedList::<BuiltNode>::default();

    let count_text = plain_text(&format!("{} {}", selected_node_ids.len(), term_labels.media_node_count_label.as_str()), "ui.inspector.count")?;
    let mut header_children = UiFixedList::<BuiltNode>::default();
    header_children.try_push(count_text).map_err(|_| PluginAssemblyError::new("ui.inspector.header-children", "header child admission failed"))?;

    let nodes: Vec<&WorkflowNode> = selected_node_ids.iter().filter_map(|node_id| projection.graph.nodes.iter().find(|node| &node.id == node_id)).collect();
    if nodes.is_empty() {
        let hint = plain_text(term_labels.select_hint.as_str(), "ui.inspector.hint")?;
        header_children.try_push(hint).map_err(|_| PluginAssemblyError::new("ui.inspector.header-children", "header child admission failed"))?;
    }
    let header = semio_framework_ui_contract::section(fixed_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "ui.inspector.header-label")?)
        .default_open(true)
        .try_id("s-play-inspector.header")
        .map_err(|_| PluginAssemblyError::new("ui.inspector.header-id", "header section id admission failed"))?
        .try_children(header_children)
        .map_err(|_| PluginAssemblyError::new("ui.inspector.header-section-children", "header section child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.inspector.header-section", "header section admission failed"))?;
    sections.try_push(header).map_err(|_| PluginAssemblyError::new("ui.inspector.sections", "inspector section admission failed"))?;

    if !nodes.is_empty() {
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());

        let mut node_fields = UiFixedList::<BuiltNode>::default();
        if selected_node_ids.len() == 1 {
            let noop = crate::engine::space::s_play_action("noOperation", None)?;
            let control = semio_framework_ui_contract::input(InputKind::Text)
                .value(fixed_text(&selected_node_ids[0], "ui.inspector.id-text")?)
                .try_id("s-play-inspector.media-node.id.input")
                .map_err(|_| PluginAssemblyError::new("ui.inspector.id-input-id", "node id input id admission failed"))?;
            let control = bind_action(control, Trigger::Change, noop)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.id-input", "node id input admission failed"))?;
            node_fields
                .try_push(field_node(term_labels.node_id.as_str(), "ui.inspector.id-field", "s-play-inspector.media-node.id".into(), control)?)
                .map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;
        }

        // 📊️ Coordinate-axis notation, identical in every locale — genuine runtime/technical data,
        // not translatable prose.
        let x_action = crate::engine::space::s_play_action("patchMediaNodes", Some(ui_value_map([("nodeIds", node_ids_arg(selected_node_ids)?), ("field", ui_value_text("position")?), ("axis", ui_value_text("x")?)])?))?;
        let x_value = if x_uniform { xs.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() };
        let mut x_control = semio_framework_ui_contract::input(InputKind::Number).value(fixed_text(&x_value, "ui.inspector.x-text")?);
        if !x_uniform {
            x_control = x_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.x-placeholder")?);
        }
        let x_control = x_control.try_id("s-play-inspector.media-node.x.input").map_err(|_| PluginAssemblyError::new("ui.inspector.x-input-id", "x input id admission failed"))?;
        let x_control = bind_action(x_control, Trigger::Change, x_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.x-input", "x input admission failed"))?;
        node_fields.try_push(field_node("X", "ui.inspector.x-field", "s-play-inspector.media-node.x".into(), x_control)?).map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;

        let y_action = crate::engine::space::s_play_action("patchMediaNodes", Some(ui_value_map([("nodeIds", node_ids_arg(selected_node_ids)?), ("field", ui_value_text("position")?), ("axis", ui_value_text("y")?)])?))?;
        let y_value = if y_uniform { ys.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() };
        let mut y_control = semio_framework_ui_contract::input(InputKind::Number).value(fixed_text(&y_value, "ui.inspector.y-text")?);
        if !y_uniform {
            y_control = y_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.y-placeholder")?);
        }
        let y_control = y_control.try_id("s-play-inspector.media-node.y.input").map_err(|_| PluginAssemblyError::new("ui.inspector.y-input-id", "y input id admission failed"))?;
        let y_control = bind_action(y_control, Trigger::Change, y_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.y-input", "y input admission failed"))?;
        node_fields.try_push(field_node("Y", "ui.inspector.y-field", "s-play-inspector.media-node.y".into(), y_control)?).map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;

        let media_nodes_label = if selected_node_ids.len() == 1 { term_labels.workflow_node.as_str().to_string() } else { format!("{} ({})", term_labels.workflow_nodes.as_str(), selected_node_ids.len()) };
        let media_nodes_section = semio_framework_ui_contract::section(fixed_label(&media_nodes_label, "ui.inspector.media-nodes-label")?)
            .default_open(true)
            .try_id("s-play-inspector.media-nodes")
            .map_err(|_| PluginAssemblyError::new("ui.inspector.media-nodes-id", "media-nodes section id admission failed"))?
            .try_children(node_fields)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.media-nodes-children", "media-nodes section child admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.inspector.media-nodes-section", "media-nodes section admission failed"))?;
        sections.try_push(media_nodes_section).map_err(|_| PluginAssemblyError::new("ui.inspector.sections", "inspector section admission failed"))?;

        let node_labels: Vec<_> = nodes.iter().map(|node| node.label.clone()).collect();
        let programs: Vec<_> = nodes.iter().map(|node| node.plugin_id.clone()).collect();
        let apps: Vec<_> = nodes.iter().map(|node| node.app_id.clone()).collect();
        let label_uniform = ui_inspector_all_equal(&node_labels);
        let program_uniform = ui_inspector_all_equal(&programs);
        let app_uniform = ui_inspector_all_equal(&apps);

        let mut instance_fields = UiFixedList::<BuiltNode>::default();
        instance_fields
            .try_push(plain_text(
                &format!("{}: {}", term_labels.program_prefix.as_str(), if program_uniform { programs.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() }),
                "ui.inspector.program-text",
            )?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        instance_fields
            .try_push(plain_text(
                &format!("{}: {}", term_labels.app_prefix.as_str(), if app_uniform { apps.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() }),
                "ui.inspector.app-text",
            )?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        if selected_node_ids.len() == 1 {
            instance_fields
                .try_push(plain_text(&format!("{}: {}", term_labels.instance_id_prefix.as_str(), selected_node_ids[0]), "ui.inspector.instance-id-text")?)
                .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        }

        let node_ids_for_label = node_ids_arg(selected_node_ids)?;
        let label_action = crate::engine::space::s_play_action("patchAppInstances", Some(ui_value_map([("nodeIds", node_ids_for_label), ("field", ui_value_text("label")?)])?))?;
        let label_value = if label_uniform { node_labels.first().cloned().unwrap_or_default() } else { String::new() };
        let mut label_control = semio_framework_ui_contract::input(InputKind::Text).value(fixed_text(&label_value, "ui.inspector.label-text")?);
        if !label_uniform {
            label_control = label_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.label-placeholder")?);
        }
        let label_control = label_control.try_id("s-play-inspector.app-instance.label.input").map_err(|_| PluginAssemblyError::new("ui.inspector.label-input-id", "label input id admission failed"))?;
        let label_control = bind_action(label_control, Trigger::Change, label_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.label-input", "label input admission failed"))?;
        instance_fields
            .try_push(field_node(term_labels.label.as_str(), "ui.inspector.label-field", "s-play-inspector.app-instance.label".into(), label_control)?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;

        if selected_node_ids.len() == 1 {
            if let Some(node) = nodes.first() {
                if let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) {
                    for field_spec in &registration.parameter_fields {
                        let binding = projection.parameter_bindings.iter().find(|entry| entry.node_id == node.id && entry.field_path == field_spec.field_path);
                        let mut compatible = Vec::new();
                        for parameter in &projection.parameters {
                            if crate::engine::space::engine::resolve_future(os_parameter_types_compatible_shim(parameter, &field_spec.parameter_type)) {
                                compatible.push(parameter);
                            }
                        }
                        let current_value = binding.map_or_else(|| "__direct__".to_string(), |entry| entry.parameter_id.clone());
                        let mut select_builder = semio_framework_ui_contract::select(fixed_text(&current_value, "ui.inspector.bind-select-value")?)
                            .try_id(format!("s-play-inspector.app-parameter.{}.select", field_spec.field_path))
                            .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-id", "parameter-binding select id admission failed"))?;
                        select_builder = select_builder
                            .try_item(fixed_text("__direct__", "ui.inspector.bind-select-item-value")?, fixed_label(term_labels.direct_value.as_str(), "ui.inspector.bind-select-item-label")?)
                            .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-item", "parameter-binding select item admission failed"))?;
                        for parameter in compatible {
                            let value_id = entity_id(parameter);
                            let name = match parameter {
                                WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
                            };
                            select_builder = select_builder
                                .try_item(fixed_text(&value_id, "ui.inspector.bind-select-item-value")?, fixed_label(&name, "ui.inspector.bind-select-item-label")?)
                                .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-item", "parameter-binding select item admission failed"))?;
                        }
                        let bind_action_pair =
                            crate::engine::space::s_play_action("bindParameterField", Some(ui_value_map([("nodeId", ui_value_text(node.id.as_str())?), ("fieldPath", ui_value_text(field_spec.field_path.as_str())?)])?))?;
                        let select_node = bind_action(select_builder, Trigger::Change, bind_action_pair)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select", "parameter-binding select admission failed"))?;
                        instance_fields
                            .try_push(field_node(&field_spec.label, "ui.inspector.bind-field", format!("s-play-inspector.app-parameter.{}", field_spec.field_path), select_node)?)
                            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
                        if let Some(binding) = binding {
                            let mut bound_parameter = None;
                            for entry in &projection.parameters {
                                if entity_id(entry) == binding.parameter_id {
                                    bound_parameter = Some(entry);
                                    break;
                                }
                            }
                            if let Some(parameter) = bound_parameter {
                                let os_value = os_parameter_value(&crate::engine::space::engine::resolve_future(workflow_parameter_to_os(parameter)));
                                instance_fields
                                    .try_push(plain_text(&format!("{}: {}", term_labels.bound_value_prefix.as_str(), os_value), "ui.inspector.bound-value-text")?)
                                    .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
                            }
                        }
                    }
                }
            }
        }

        let app_instances_label = if selected_node_ids.len() == 1 { term_labels.app_instance.as_str().to_string() } else { format!("{} ({})", term_labels.app_instances.as_str(), selected_node_ids.len()) };
        let app_instances_section = semio_framework_ui_contract::section(fixed_label(&app_instances_label, "ui.inspector.app-instances-label")?)
            .default_open(true)
            .try_id("s-play-inspector.app-instances")
            .map_err(|_| PluginAssemblyError::new("ui.inspector.app-instances-id", "app-instances section id admission failed"))?
            .try_children(instance_fields)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.app-instances-children", "app-instances section child admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.inspector.app-instances-section", "app-instances section admission failed"))?;
        sections.try_push(app_instances_section).map_err(|_| PluginAssemblyError::new("ui.inspector.sections", "inspector section admission failed"))?;
    }

    semio_framework_ui_contract::column()
        .try_children(sections)
        .map_err(|_| PluginAssemblyError::new("ui.inspector.children", "inspector panel child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.inspector.build", "inspector panel admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
        let node = render(&projection, &ids, semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale)).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("s-play-inspector.app-instance.label"), "label field id must reach the tree: {json}");
        assert!(json.contains("patchAppInstances"), "label field action must reach the tree: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_selection_renders_the_select_hint_only() {
        let projection = demo_space_projection().await;
        let node = render(&projection, &[], &SStudioLabels::NATIVE_EN).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("s-play-inspector.header"), "header section must always render: {json}");
        assert!(!json.contains("s-play-inspector.media-nodes"), "no selection must not render the media-nodes section: {json}");
    }
}
//#endregion 🧪️Tests
