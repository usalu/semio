//! 🔍️ Block 3D play app panel — the inspector: the object kind's identity fields, active-representation
//! select, plus a vortex count.

use crate::Block3dSnapshot;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::{block3d_action, ui_label, ui_value_map, ui_value_text};
use semio_framework_plugin::{
    Buildable, BuiltNode, HasBase, HasChildren, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, Trigger, UiAssemblyResult, UiText,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_ui_contract::{self as ui, InputKind};

//#region 🔖️Constants
pub const BLOCK3D_BODY_INSPECTOR: &str = "block3d.play.inspector";
pub const BLOCK3D_INSPECTOR_SUMMARY: &str = "block3d-play-inspector.summary";
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
    ui::tree_item(ui_label(label)?)
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

/// 🧾️ One inspector row recorded as DATA — the summary section's window decides how many of these
/// get built, so a field is never materialised for a viewport that will not show it
/// (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
enum InspectorField<'a> {
    /// ✏️ A `blur`-committed text control bound to `patchObjectKind` for `document_field`.
    Text { id: &'a str, label: &'a str, value: &'a str, document_field: &'static str },
    /// 🧱️ The active-representation picker, built from the live document.
    Representation { label: &'a str },
    /// 🔒️ A disabled text control, no binding.
    Readonly { id: &'a str, label: &'a str, value: String },
}

/// 🪟️ The inspector's one summary section is WINDOWED like every other tree container: it stamps its
/// full field count and materialises only the rows its window asks for.
pub fn render(definition: &Block3dSnapshot, active_representation_id: Option<&str>, labels: &Block3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let fields = [
        InspectorField::Text { id: "block3d-play-inspector.name", label: labels.name.as_str(), value: &definition.object_kind.name, document_field: "name" },
        InspectorField::Text { id: "block3d-play-inspector.label", label: labels.label.as_str(), value: &definition.object_kind.label, document_field: "label" },
        InspectorField::Representation { label: labels.representation.as_str() },
        InspectorField::Readonly { id: "block3d-play-inspector.vortex-count", label: labels.vortices.as_str(), value: definition.vortices.len().to_string() },
    ];
    PanelTreeBuilder::new("block3d-play-inspector")?
        .window_section(windows, BLOCK3D_INSPECTOR_SUMMARY, Some(ui_label(labels.summary.as_str())?), true, &fields, |field| match field {
            InspectorField::Text { id, label, value, document_field } => text_field(id, label, value, *document_field),
            InspectorField::Representation { label } => representation_field(definition, active_representation_id, label),
            InspectorField::Readonly { id, label, value } => readonly_field(id, label, value.as_str()),
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
