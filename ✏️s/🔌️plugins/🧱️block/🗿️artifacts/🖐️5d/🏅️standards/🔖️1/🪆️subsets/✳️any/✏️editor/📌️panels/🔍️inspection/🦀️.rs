//! 🔍️ Block 5D play app panel — the inspector: the part kind's identity fields plus a grip count.

use crate::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::{block5d_action, ui_label, ui_value_map, ui_value_text};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{
    BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
// 🏗️ The contract's own node BUILDERS, reached directly rather than through
// `plugin_app_close_prelude`: that prelude re-exports the SDK's `tree_item(id, label) -> BuiltNode`
// explicitly, which shadows the contract's `tree_item(label) -> TreeItemBuilder` glob — the builder
// is what a field row needs, since it nests its control as a child.
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BLOCK5D_BODY_INSPECTOR: &str = "block5d.play.inspector";
pub const BLOCK5D_INSPECTOR_SUMMARY: &str = "block5d-play-inspector.summary";
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
    ui::tree_item(ui_label(label)?)
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

/// 🧾️ One inspector row recorded as DATA — the summary section's window decides how many of these
/// get built, so a field is never materialised for a viewport that will not show it
/// (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
enum InspectorField<'a> {
    /// ✏️ A `blur`-committed text control bound to `patchPartKind` for `document_field`.
    Text { id: &'a str, label: &'a str, value: &'a str, document_field: &'static str },
    /// 🔒️ A disabled text control, no binding.
    Readonly { id: &'a str, label: &'a str, value: String },
}

fn inspector_row(field: &InspectorField<'_>) -> UiAssemblyResult<BuiltNode> {
    match field {
        InspectorField::Text { id, label, value, document_field } => text_field(id, label, value, *document_field),
        InspectorField::Readonly { id, label, value } => readonly_field(id, label, value.as_str()),
    }
}

/// 🪟️ The inspector's one summary section is WINDOWED like every other tree container: it stamps its
/// full field count and materialises only the rows its window asks for.
pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let fields = [
        InspectorField::Text { id: "block5d-play-inspector.name", label: labels.name.as_str(), value: &definition.part_kind.name, document_field: "name" },
        InspectorField::Text { id: "block5d-play-inspector.label", label: labels.label.as_str(), value: &definition.part_kind.label, document_field: "label" },
        InspectorField::Readonly { id: "block5d-play-inspector.grip-count", label: labels.grips.as_str(), value: definition.grips.len().to_string() },
    ];
    PanelTreeBuilder::new("block5d-play-inspector")?
        .window_section(windows, BLOCK5D_INSPECTOR_SUMMARY, Some(ui_label(labels.summary.as_str())?), true, &fields, inspector_row)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
