//! 🔍️ DAG play app panel — the per-node inspector (name/kind/id plus slider-specific fields).

use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::{dag_action, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::dag::terminology::DagPlayLabels;
use infinite_board_port_directed_dag::{dag_node_kind_tag, DagNodeKind, DagNodeSpec};
use semio_framework_plugin::{ui_inspector_mixed_number, ui_inspector_mixed_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER};
use semio_framework_plugin::plugin_app_close_prelude::{column, field, input, section, text, Buildable, BuiltNode, HasBase, HasChildren, InputKind, Label, Trigger, UiText};

//#region 🔖️Constants
pub const DAG_PLAY_BODY_INSPECTOR: &str = "dag.play.inspection";
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

fn field_node(id: &str, title: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    field(label(title)?).try_id(id).map_err(|_| admission_error())?.try_child(control).map_err(|_| admission_error())?.try_build().map_err(|_| admission_error())
}

fn readonly_field(id: &str, title: &str, value: &str) -> UiAssemblyResult<BuiltNode> {
    let control = text(label(value)?).try_build().map_err(|_| admission_error())?;
    field_node(id, title, control)
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
    field_node(id, title, control.try_build().map_err(|_| admission_error())?)
}

fn patch_args(node_ids: &[String], field: &str) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
    let ids = node_ids.iter().map(ui_value_text).collect::<UiAssemblyResult<Vec<_>>>()?;
    ui_value_map([("nodeIds", ui_value_list(ids)?), ("field", ui_value_text(field)?)])
}

fn group_node(id: &str, title: &str, fields: Vec<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    section(label(title)?).try_id(id).map_err(|_| admission_error())?.try_children(fields).map_err(|_| admission_error())?.try_build().map_err(|_| admission_error())
}
//#endregion 🔖️Fields

//#region 🔖️Render
pub fn render(document: &DagSnapshot, selected: &[String], labels: &DagPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let owned_nodes = document.nodes();
    let nodes: Vec<&DagNodeSpec> = selected.iter().filter_map(|id| owned_nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        let (id, title) = if selected.is_empty() { ("dag-play-inspector.empty", labels.select_a_node) } else { ("dag-play-inspector.missing", labels.node_not_found) };
        let content = text(label(title.as_str())?).try_build().map_err(|_| admission_error())?;
        return group_node(id, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, vec![content]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        let mut fields = Vec::new();
        for (name, title) in [("value", labels.field_value), ("min", labels.field_min), ("max", labels.field_max)] {
            let values = nodes.iter().filter_map(|node| match node.kind {
                DagNodeKind::Slider { value, min, max, .. } => Some(match name { "min" => min, "max" => max, _ => value }),
                _ => None,
            }).collect::<Vec<_>>();
            let mixed = ui_inspector_mixed_number(&values);
            fields.push(input_field(&format!("dag-play-inspector.slider-{name}"), title.as_str(), InputKind::Number,
                &if mixed.uniform { mixed.value.to_string() } else { String::new() },
                (!mixed.uniform).then_some(UI_INSPECTOR_MIXED_PLACEHOLDER), "patchDagNodes", patch_args(&node_ids, name)?)?);
        }
        groups.push(group_node("dag-play-inspector.kind.slider", labels.slider_group.as_str(), fields)?);
    }
    let id_field = if node_ids.len() == 1 {
        input_field("dag-play-inspector.id", labels.field_id.as_str(), InputKind::Text, &node_ids[0], None, "renameDagNode", ui_value_map([("oldId", ui_value_text(&node_ids[0])?)])?)?
    } else {
        readonly_field("dag-play-inspector.id", labels.field_id.as_str(), &format!("{} {}", node_ids.len(), labels.selected_suffix.as_str()))?
    };
    let names = nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>();
    let mixed = ui_inspector_mixed_text(&names);
    let name_field = input_field("dag-play-inspector.name", labels.field_name.as_str(), InputKind::Text, &mixed.value, mixed.placeholder.as_deref(), "patchDagNodes", patch_args(&node_ids, "name")?)?;
    let kind = dag_node_kind_tag(&nodes[0].kind);
    let kind_field = readonly_field("dag-play-inspector.kind", labels.field_kind.as_str(), if nodes.iter().all(|node| dag_node_kind_tag(&node.kind) == kind) { kind } else { "—" })?;
    groups.push(group_node("dag-play-inspector.base", labels.node_group.as_str(), vec![id_field, name_field, kind_field])?);
    column().try_id("dag-play-inspector").map_err(|_| admission_error())?.try_children(groups).map_err(|_| admission_error())?.try_build().map_err(|_| admission_error())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_INSPECTOR));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_select_a_node_placeholder_when_nothing_is_selected() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, DAG_PLAY_BODY_INSPECTOR).await.contains("Select a node"));
    }

    /// 🕹️ `render` carries no `InteractionView` (`DagPlayApp::render` always calls this panel's own
    /// `render` with an empty selection now — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
    /// matching `space`'s identical gap), so this exercises the panel's OWN rendering logic directly
    /// with an explicit selection, the way `space`'s inspector test does, rather than driving it
    /// end-to-end through a (now selection-blind) app dispatch.
    #[semio_framework_async_macros::async_test]
    async fn renders_id_name_and_kind_fields_for_a_single_selected_node() {
        let document = crate::artifacts::dag::default_snapshot();
        let node_id = document.nodes().first().map(|node| node.id.clone()).expect("node");
        let labels = crate::editor::dag::terminology::dag_play_labels(&crate::editor::dag::config::DagConfig::default());
        let node = render(&document, &[node_id.clone()], labels).expect("inspector component tree");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&node_id));
        assert!(json.contains("Name") || json.contains("Kind"));
    }
}
//#endregion 🧪️Tests
