//! 🔍️ Block 3D play app panel — the inspector: the object kind's identity fields, active-representation
//! select, plus a vortex count.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::{block3d_action, ui_label, ui_node_list, ui_value_map, ui_value_text};
use semio_framework_plugin::{
    Buildable, BuiltNode, HasBase, HasChildren, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, Trigger, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_ui_contract::{self as ui, InputKind};

//#region 🔖️Constants
pub const BLOCK3D_BODY_INSPECTOR: &str = "block3d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK3D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("block3d inspector admission failed at {stage}"))
}

fn ui_value(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| inspector_error("value"))
}

/// 🏷️ Wraps one built control in its labeled field row.
fn field_row(id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    ui::field(ui_label(label)?)
        .try_id(id)
        .map_err(|_| inspector_error("field-id"))?
        .try_children([control])
        .map_err(|_| inspector_error("field-children"))?
        .try_build()
        .map_err(|_| inspector_error("field-build"))
}

/// ✏️ One editable identity row — commits on blur and dispatches `patchObjectKind` for `field`.
fn text_field(id: &str, label: &str, value: &str, field: &'static str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = block3d_action("patchObjectKind", Some(ui_value_map([("field", ui_value_text(field)?)])?))?;
    let mut input = ui::input(InputKind::Text)
        .value(ui_value(value)?)
        .commit(ui_value("blur")?)
        .try_id(format!("{id}.input"))
        .map_err(|_| inspector_error("input-id"))?;
    input = match args {
        Some(args) => input.try_on_with(Trigger::Change, action, args),
        None => input.try_on(Trigger::Change, action),
    }
    .map_err(|_| inspector_error("input-binding"))?;
    field_row(id, label, input.try_build().map_err(|_| inspector_error("input-build"))?)
}

/// 🔒️ One read-only row — a disabled text input, no action binding.
fn readonly_field(id: &str, label: &str, value: &str) -> UiAssemblyResult<BuiltNode> {
    let input = ui::input(InputKind::Text).value(ui_value(value)?).try_id(format!("{id}.input")).map_err(|_| inspector_error("readonly-id"))?.disabled(true);
    field_row(id, label, input.try_build().map_err(|_| inspector_error("readonly-build"))?)
}

/// 🧱️ The active-representation picker — one item per document representation.
fn representation_field(definition: &Block3dSnapshot, active_representation_id: Option<&str>, label: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = block3d_action("setActiveRepresentation", None)?;
    let mut select = ui::select(ui_value(active_representation_id.unwrap_or_default())?);
    for representation in &definition.representations {
        select = select.try_item(ui_value(&representation.id)?, ui_label(&representation.name)?).map_err(|_| inspector_error("select-item"))?;
    }
    select = select.try_id("block3d-play-inspector.representation").map_err(|_| inspector_error("select-id"))?;
    select = match args {
        Some(args) => select.try_on_with(Trigger::Change, action, args),
        None => select.try_on(Trigger::Change, action),
    }
    .map_err(|_| inspector_error("select-binding"))?;
    field_row("block3d-play-inspector.representation-field", label, select.try_build().map_err(|_| inspector_error("select-build"))?)
}

pub fn render(definition: &Block3dSnapshot, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiAssemblyResult<BuiltNode> {
    let rows = ui_node_list([
        text_field("block3d-play-inspector.name", labels.name.as_str(), &definition.object_kind.name, "name"),
        text_field("block3d-play-inspector.label", labels.label.as_str(), &definition.object_kind.label, "label"),
        representation_field(definition, active_representation_id, labels.representation.as_str()),
        readonly_field("block3d-play-inspector.vortex-count", labels.vortices.as_str(), &definition.vortices.len().to_string()),
    ])?;
    PanelTreeBuilder::new("block3d-play-inspector")?.section("block3d-play-inspector.summary", Some(ui_label(labels.summary.as_str())?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block3d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK3D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(json.contains("Vortices"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
