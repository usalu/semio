//! 🔍️ DAG play app panel — the per-node inspector (name/kind/id plus slider-specific fields).

use crate::editor::dag::terminology::DagPlayLabels;
use crate::editor::dag::{dag_action, ui_node_list, ui_value_list, ui_value_map, ui_value_text};
use crate::DagSnapshot;
use semio_framework_artifact_infinite_dag::{dag_node_kind_tag, DagNodeKind, DagNodeSpec};
use semio_framework_plugin::plugin_app_close_prelude::{input, Buildable, HasBase, HasChildren, InputKind, Label, Trigger, UiText};
use semio_framework_plugin::{
    tree_item_desc, ui_inspector_mixed_number, ui_inspector_mixed_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use semio_framework_ui_contract::{tree_item, BuiltNode};

//#region 🔖️Constants
pub const DAG_PLAY_BODY_INSPECTOR: &str = "dag.play.inspection";
const ROOT: &str = "dag-play-inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(DAG_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Fields
fn admission_error() -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", "DAG inspector admission failed")
}

fn label(value: &str) -> UiAssemblyResult<Label> {
    Label::try_from(value).map_err(|_| admission_error())
}

fn control_row(id: &str, title: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    tree_item(label(title)?)
        .try_id(id)
        .map_err(|_| admission_error())?
        .try_child(control)
        .map_err(|_| admission_error())?
        .try_build()
        .map_err(|_| admission_error())
}

fn input_field(id: &str, title: &str, kind: InputKind, value: &str, placeholder: Option<&str>, action: &str, args: semio_framework_plugin::UiValue) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = dag_action(action, Some(args))?;
    let mut control = input(kind).value(UiText::try_from_str(value).ok_or_else(admission_error)?).try_id(format!("{id}.input")).map_err(|_| admission_error())?;
    if let Some(placeholder) = placeholder {
        control = control.placeholder(label(placeholder)?);
    }
    if kind == InputKind::Text {
        control = control.commit(UiText::try_from_str("blur").ok_or_else(admission_error)?);
    }
    control = match args {
        Some(args) => control.try_on_with(Trigger::Change, action, args).map_err(|_| admission_error())?,
        None => control.try_on(Trigger::Change, action).map_err(|_| admission_error())?,
    };
    control_row(id, title, control.try_build().map_err(|_| admission_error())?)
}

fn patch_args(node_ids: &[String], field: &str) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
    let ids = node_ids.iter().map(ui_value_text).collect::<UiAssemblyResult<Vec<_>>>()?;
    ui_value_map([("nodeIds", ui_value_list(ids)?), ("field", ui_value_text(field)?)])
}
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(document: &DagSnapshot, selected: &[String], labels: &DagPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let owned_nodes = document.nodes();
    let nodes: Vec<&DagNodeSpec> = selected.iter().filter_map(|id| owned_nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        let (id, title) = if selected.is_empty() { ("empty", labels.select_a_node) } else { ("missing", labels.node_not_found) };
        let rows = ui_node_list([tree_item_desc(format!("{ROOT}.{id}"), label(title.as_str())?, None)?])?;
        return PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.{id}"), Some(label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build();
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut builder = PanelTreeBuilder::new(ROOT)?;
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        let mut fields = Vec::new();
        for (name, title) in [("value", labels.field_value), ("min", labels.field_min), ("max", labels.field_max)] {
            let values = nodes
                .iter()
                .filter_map(|node| match node.kind {
                    DagNodeKind::Slider { value, min, max, .. } => Some(match name {
                        "min" => min,
                        "max" => max,
                        _ => value,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let mixed = ui_inspector_mixed_number(&values);
            fields.push(input_field(
                &format!("{ROOT}.slider-{name}"),
                title.as_str(),
                InputKind::Number,
                &if mixed.uniform { mixed.value.to_string() } else { String::new() },
                (!mixed.uniform).then_some(UI_INSPECTOR_MIXED_PLACEHOLDER),
                "patchDagNodes",
                patch_args(&node_ids, name)?,
            )?);
        }
        builder = builder.section(format!("{ROOT}.kind.slider"), Some(label(labels.slider_group.as_str())?), true, ui_node_list(fields)?)?;
    }
    let id_row = if node_ids.len() == 1 {
        input_field(&format!("{ROOT}.id"), labels.field_id.as_str(), InputKind::Text, &node_ids[0], None, "renameDagNode", ui_value_map([("oldId", ui_value_text(&node_ids[0])?)])?)?
    } else {
        tree_item_desc(format!("{ROOT}.id"), label(labels.field_id.as_str())?, Some(format!("{} {}", node_ids.len(), labels.selected_suffix.as_str())))?
    };
    let names = nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>();
    let mixed = ui_inspector_mixed_text(&names);
    let name_row = input_field(&format!("{ROOT}.name"), labels.field_name.as_str(), InputKind::Text, &mixed.value, mixed.placeholder.as_deref(), "patchDagNodes", patch_args(&node_ids, "name")?)?;
    let kind = dag_node_kind_tag(&nodes[0].kind);
    let kind_value = if nodes.iter().all(|node| dag_node_kind_tag(&node.kind) == kind) { kind.to_string() } else { "—".to_string() };
    let kind_row = tree_item_desc(format!("{ROOT}.kind"), label(labels.field_kind.as_str())?, Some(kind_value))?;
    builder.section(format!("{ROOT}.base"), Some(label(labels.node_group.as_str())?), true, ui_node_list([id_row, name_row, kind_row])?)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
