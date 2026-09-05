//! 🔍️ Block 5D play app panel — the inspector: the part kind's identity fields plus a grip count.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::{block5d_action, ui_label, ui_node_list, ui_value_map, ui_value_text};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{
    BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
// 🚧️ SDK GAP: the block crate has no direct `semio-framework-ui-contract` dependency (unlike puzzle/
// lowpoly), so the contract's node builders are reached through the plugin SDK's own re-export.
use semio_framework_plugin::plugin_app_close_prelude as ui;

//#region 🔖️Constants
pub const BLOCK5D_BODY_INSPECTOR: &str = "block5d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK5D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("block5d inspector admission failed at {stage}"))
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

/// ✏️ One editable identity row — commits on blur and dispatches `patchPartKind` for `field`.
fn text_field(id: &str, label: &str, value: &str, field: &'static str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = block5d_action("patchPartKind", Some(ui_value_map([("field", ui_value_text(field)?)])?))?;
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

pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels) -> UiAssemblyResult<BuiltNode> {
    let rows = ui_node_list([
        text_field("block5d-play-inspector.name", labels.name.as_str(), &definition.part_kind.name, "name"),
        text_field("block5d-play-inspector.label", labels.label.as_str(), &definition.part_kind.label, "label"),
        readonly_field("block5d-play-inspector.grip-count", labels.grips.as_str(), &definition.grips.len().to_string()),
    ])?;
    PanelTreeBuilder::new("block5d-play-inspector")?.section("block5d-play-inspector.summary", Some(ui_label(labels.summary.as_str())?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block5d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK5D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
