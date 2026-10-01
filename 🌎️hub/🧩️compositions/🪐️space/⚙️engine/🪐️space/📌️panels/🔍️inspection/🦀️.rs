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
use semio_framework_os::{os_app_registration, WorkflowNode, WorkflowParameter, WorkflowSnapshot};
use semio_framework_plugin::{
    tree_item, tree_item_desc, ui_inspector_all_equal, ActionId, Buildable, HasBase, HasChildren, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use semio_framework_ui_contract::{input, select, tree_item as tree_item_row, BuiltNode, InputKind, Label, Trigger};

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
const ROOT: &str = "s-play-inspector";

fn fixed_text(value: &str, code: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new(code, "fixed inspector text admission failed"))
}

fn fixed_label(value: &str, code: &'static str) -> UiAssemblyResult<Label> {
    Label::try_from(value).map_err(|_| PluginAssemblyError::new(code, "fixed inspector label admission failed"))
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

fn control_row(id: &str, label: &str, code: &'static str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    tree_item_row(fixed_label(label, code)?)
        .try_id(id)
        .map_err(|_| PluginAssemblyError::new(code, "inspector row id admission failed"))?
        .try_child(control)
        .map_err(|_| PluginAssemblyError::new(code, "inspector row child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new(code, "inspector row admission failed"))
}

fn read_only_row(id: &str, label: &str, code: &'static str, value: impl Into<String>) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(id, fixed_label(label, code)?, Some(value.into()))
}

/// 🪟️ One row of the selected instance's parameter-binding block — the windowed unit of the only
/// open-ended list in this panel (`os_app_registration(...).parameter_fields`, which is a per-app
/// schema, not a document collection, but still unbounded from this panel's side). A field always
/// contributes its `Binding` select row and, when a parameter is actually bound to it, a read-only
/// `Bound` row underneath; flattening both into ONE list is what lets the window address them by
/// index without changing the rendered row order.
enum InspectorParameterRow<'a> {
    Binding(&'a semio_framework_os::OsParameterFieldSpec),
    Bound { field_path: &'a str, value: String },
}

fn workflow_parameter_display_value(parameter: &WorkflowParameter) -> String {
    match parameter {
        WorkflowParameter::Numeric { value, .. } => value.to_string(),
        WorkflowParameter::Categorical { value, .. } => value.clone(),
        WorkflowParameter::Toggle { value, .. } => value.to_string(),
        WorkflowParameter::Text { value, .. } => value.clone(),
    }
}

/// 🪟️ The flat row list the parameter-binding section windows: one `Binding` row per declared field,
/// each followed by a `Bound` row when that field currently resolves to a workflow parameter. Cheap to
/// build whole (string lookups only) — the expensive per-row `select` assembly happens only for the
/// rows the window actually materialises.
fn inspector_parameter_rows<'a>(projection: &'a WorkflowSnapshot, node: &WorkflowNode, registration: &'a semio_framework_os::OsAppRegistration) -> Vec<InspectorParameterRow<'a>> {
    let mut rows = Vec::new();
    for field_spec in &registration.parameter_fields {
        rows.push(InspectorParameterRow::Binding(field_spec));
        let Some(binding) = projection.parameter_bindings.iter().find(|entry| entry.node_id == node.id && entry.field_path == field_spec.field_path) else { continue };
        let Some(parameter) = projection.parameters.iter().find(|entry| entity_id(entry) == binding.parameter_id) else { continue };
        rows.push(InspectorParameterRow::Bound { field_path: field_spec.field_path.as_str(), value: workflow_parameter_display_value(parameter) });
    }
    rows
}

fn inspector_parameter_row(projection: &WorkflowSnapshot, node: &WorkflowNode, term_labels: &SStudioLabels, row: &InspectorParameterRow<'_>) -> UiAssemblyResult<BuiltNode> {
    let field_spec = match row {
        InspectorParameterRow::Bound { field_path, value } => {
            return read_only_row(&format!("{ROOT}.app-parameter.{field_path}.bound"), term_labels.bound_value_prefix.as_str(), "ui.inspector.bound-value-text", value.clone());
        }
        InspectorParameterRow::Binding(field_spec) => field_spec,
    };
    let binding = projection.parameter_bindings.iter().find(|entry| entry.node_id == node.id && entry.field_path == field_spec.field_path);
    let current_value = binding.map_or_else(|| "__direct__".to_string(), |entry| entry.parameter_id.clone());
    let mut select_builder = select(fixed_text(&current_value, "ui.inspector.bind-select-value")?)
        .try_id(format!("{ROOT}.app-parameter.{}.select", field_spec.field_path))
        .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-id", "parameter-binding select id admission failed"))?;
    select_builder = select_builder
        .try_item(fixed_text("__direct__", "ui.inspector.bind-select-item-value")?, fixed_label(term_labels.direct_value.as_str(), "ui.inspector.bind-select-item-label")?)
        .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-item", "parameter-binding select item admission failed"))?;
    for parameter in &projection.parameters {
        if !crate::engine::space::engine::resolve_future(os_parameter_types_compatible_shim(parameter, &field_spec.parameter_type)) {
            continue;
        }
        let value_id = entity_id(parameter);
        let name = match parameter {
            WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
        };
        select_builder = select_builder
            .try_item(fixed_text(&value_id, "ui.inspector.bind-select-item-value")?, fixed_label(&name, "ui.inspector.bind-select-item-label")?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select-item", "parameter-binding select item admission failed"))?;
    }
    let bind_action_pair = crate::engine::space::s_play_action("bindParameterField", Some(ui_value_map([("nodeId", ui_value_text(node.id.as_str())?), ("fieldPath", ui_value_text(field_spec.field_path.as_str())?)])?))?;
    let select_node = bind_action(select_builder, Trigger::Change, bind_action_pair)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.bind-select", "parameter-binding select admission failed"))?;
    control_row(&format!("{ROOT}.app-parameter.{}", field_spec.field_path), &field_spec.label, "ui.inspector.bind-field", select_node)
}

pub fn render(projection: &WorkflowSnapshot, selected_node_ids: &[String], term_labels: &SStudioLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let nodes: Vec<&WorkflowNode> = selected_node_ids.iter().filter_map(|node_id| projection.graph.nodes.iter().find(|node| &node.id == node_id)).collect();

    let mut header_rows = semio_framework_plugin::UiFixedList::<BuiltNode>::default();
    header_rows
        .try_push(tree_item(
            format!("{ROOT}.header.count"),
            fixed_label(&format!("{} {}", selected_node_ids.len(), term_labels.media_node_count_label.as_str()), "ui.inspector.count")?,
        )?)
        .map_err(|_| PluginAssemblyError::new("ui.inspector.header-children", "header child admission failed"))?;
    if nodes.is_empty() {
        header_rows
            .try_push(tree_item_desc(format!("{ROOT}.header.hint"), fixed_label(term_labels.select_hint.as_str(), "ui.inspector.hint")?, None)?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.header-children", "header child admission failed"))?;
    }

    let mut builder = PanelTreeBuilder::new(ROOT)?.section(
        format!("{ROOT}.header"),
        Some(fixed_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "ui.inspector.header-label")?),
        true,
        header_rows,
    )?;

    if !nodes.is_empty() {
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());

        let mut node_fields = semio_framework_plugin::UiFixedList::<BuiltNode>::default();
        if selected_node_ids.len() == 1 {
            let noop = crate::engine::space::s_play_action("noOperation", None)?;
            let control = input(InputKind::Text)
                .value(fixed_text(&selected_node_ids[0], "ui.inspector.id-text")?)
                .try_id(format!("{ROOT}.media-node.id.input"))
                .map_err(|_| PluginAssemblyError::new("ui.inspector.id-input-id", "node id input id admission failed"))?;
            let control = bind_action(control, Trigger::Change, noop)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.id-input", "node id input admission failed"))?;
            node_fields
                .try_push(control_row(&format!("{ROOT}.media-node.id"), term_labels.node_id.as_str(), "ui.inspector.id-field", control)?)
                .map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;
        }

        let x_action = crate::engine::space::s_play_action("patchMediaNodes", Some(ui_value_map([("nodeIds", node_ids_arg(selected_node_ids)?), ("field", ui_value_text("position")?), ("axis", ui_value_text("x")?)])?))?;
        let x_value = if x_uniform { xs.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() };
        let mut x_control = input(InputKind::Number).value(fixed_text(&x_value, "ui.inspector.x-text")?);
        if !x_uniform {
            x_control = x_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.x-placeholder")?);
        }
        let x_control = x_control.try_id(format!("{ROOT}.media-node.x.input")).map_err(|_| PluginAssemblyError::new("ui.inspector.x-input-id", "x input id admission failed"))?;
        let x_control = bind_action(x_control, Trigger::Change, x_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.x-input", "x input admission failed"))?;
        node_fields.try_push(control_row(&format!("{ROOT}.media-node.x"), "X", "ui.inspector.x-field", x_control)?).map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;

        let y_action = crate::engine::space::s_play_action("patchMediaNodes", Some(ui_value_map([("nodeIds", node_ids_arg(selected_node_ids)?), ("field", ui_value_text("position")?), ("axis", ui_value_text("y")?)])?))?;
        let y_value = if y_uniform { ys.first().map(|v| v.to_string()).unwrap_or_default() } else { String::new() };
        let mut y_control = input(InputKind::Number).value(fixed_text(&y_value, "ui.inspector.y-text")?);
        if !y_uniform {
            y_control = y_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.y-placeholder")?);
        }
        let y_control = y_control.try_id(format!("{ROOT}.media-node.y.input")).map_err(|_| PluginAssemblyError::new("ui.inspector.y-input-id", "y input id admission failed"))?;
        let y_control = bind_action(y_control, Trigger::Change, y_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.y-input", "y input admission failed"))?;
        node_fields.try_push(control_row(&format!("{ROOT}.media-node.y"), "Y", "ui.inspector.y-field", y_control)?).map_err(|_| PluginAssemblyError::new("ui.inspector.node-fields", "media-node field admission failed"))?;

        let media_nodes_label = if selected_node_ids.len() == 1 { term_labels.workflow_node.as_str().to_string() } else { format!("{} ({})", term_labels.workflow_nodes.as_str(), selected_node_ids.len()) };
        builder = builder.section(format!("{ROOT}.media-nodes"), Some(fixed_label(&media_nodes_label, "ui.inspector.media-nodes-label")?), true, node_fields)?;

        let node_labels: Vec<_> = nodes.iter().map(|node| node.label.clone()).collect();
        let programs: Vec<_> = nodes.iter().map(|node| node.plugin_id.clone()).collect();
        let apps: Vec<_> = nodes.iter().map(|node| node.app_id.clone()).collect();
        let label_uniform = ui_inspector_all_equal(&node_labels);
        let program_uniform = ui_inspector_all_equal(&programs);
        let app_uniform = ui_inspector_all_equal(&apps);

        let mut instance_fields = semio_framework_plugin::UiFixedList::<BuiltNode>::default();
        instance_fields
            .try_push(read_only_row(
                &format!("{ROOT}.app-instance.program"),
                term_labels.program_prefix.as_str(),
                "ui.inspector.program-text",
                if program_uniform { programs.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() },
            )?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        instance_fields
            .try_push(read_only_row(
                &format!("{ROOT}.app-instance.app"),
                term_labels.app_prefix.as_str(),
                "ui.inspector.app-text",
                if app_uniform { apps.first().cloned().unwrap_or_default() } else { term_labels.mixed_placeholder.as_str().to_string() },
            )?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        if selected_node_ids.len() == 1 {
            instance_fields
                .try_push(read_only_row(&format!("{ROOT}.app-instance.instance-id"), term_labels.instance_id_prefix.as_str(), "ui.inspector.instance-id-text", selected_node_ids[0].clone())?)
                .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;
        }

        let node_ids_for_label = node_ids_arg(selected_node_ids)?;
        let label_action = crate::engine::space::s_play_action("patchAppInstances", Some(ui_value_map([("nodeIds", node_ids_for_label), ("field", ui_value_text("label")?)])?))?;
        let label_value = if label_uniform { node_labels.first().cloned().unwrap_or_default() } else { String::new() };
        let mut label_control = input(InputKind::Text).value(fixed_text(&label_value, "ui.inspector.label-text")?);
        if !label_uniform {
            label_control = label_control.placeholder(fixed_label(term_labels.mixed_placeholder.as_str(), "ui.inspector.label-placeholder")?);
        }
        let label_control = label_control.try_id(format!("{ROOT}.app-instance.label.input")).map_err(|_| PluginAssemblyError::new("ui.inspector.label-input-id", "label input id admission failed"))?;
        let label_control = bind_action(label_control, Trigger::Change, label_action)?.try_build().map_err(|_| PluginAssemblyError::new("ui.inspector.label-input", "label input admission failed"))?;
        instance_fields
            .try_push(control_row(&format!("{ROOT}.app-instance.label"), term_labels.label.as_str(), "ui.inspector.label-field", label_control)?)
            .map_err(|_| PluginAssemblyError::new("ui.inspector.instance-fields", "app-instance field admission failed"))?;

        let app_instances_label = if selected_node_ids.len() == 1 { term_labels.app_instance.as_str().to_string() } else { format!("{} ({})", term_labels.app_instances.as_str(), selected_node_ids.len()) };
        builder = builder.section(format!("{ROOT}.app-instances"), Some(fixed_label(&app_instances_label, "ui.inspector.app-instances-label")?), true, instance_fields)?;

        let registration = (selected_node_ids.len() == 1).then(|| nodes.first().and_then(|node| os_app_registration(&node.plugin_id, &node.app_id))).flatten();
        if let (Some(registration), Some(node)) = (&registration, nodes.first()) {
            let parameter_rows = inspector_parameter_rows(projection, node, registration);
            builder = builder.window_section(
                windows,
                &format!("{ROOT}.app-parameters"),
                Some(fixed_label(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, "ui.inspector.app-parameters-label")?),
                true,
                &parameter_rows,
                |row| inspector_parameter_row(projection, node, term_labels, row),
            )?;
        }
    }

    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
