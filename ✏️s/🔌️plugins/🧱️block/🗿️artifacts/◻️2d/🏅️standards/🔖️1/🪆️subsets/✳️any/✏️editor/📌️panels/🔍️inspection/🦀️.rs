//! 🔍️ Block 2D play app panel — the inspector: the node kind's identity fields plus a handle count.

use crate::Block2dSnapshot;
use crate::editor::block2d::terminology::Block2dLabels;
use crate::editor::block2d::{block2d_action, ui_label, ui_text, ui_value_map, ui_value_text};
use semio_framework_plugin::plugin_app_close_prelude::{input, Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BLOCK2D_BODY_INSPECTOR: &str = "block2d.play.inspector";
pub const BLOCK2D_INSPECTOR_SUMMARY: &str = "block2d-play-inspector.summary";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK2D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn admission(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", stage)
}

/// ✍️ One editable identity field: a `blur`-committed text control bound to `patchNodeKind` with the
/// document field name as its only argument (the React interpreter merges `{ value }` at commit).
fn text_field(id: &str, label: &str, value: &str, document_field: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = block2d_action("patchNodeKind", Some(ui_value_map([("field", ui_value_text(document_field)?)])?))?;
    let control = input(InputKind::Text).value(ui_text(value)?).commit(ui_text("blur")?).try_id(format!("{id}.input")).map_err(|_| admission("block2d inspector control id admission failed"))?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Change, action, args),
        None => control.try_on(Trigger::Change, action),
    }
    .map_err(|_| admission("block2d inspector control binding admission failed"))?
    .try_build()
    .map_err(|_| admission("block2d inspector control admission failed"))?;
    semio_framework_ui_contract::tree_item(ui_label(label)?)
        .try_id(id)
        .map_err(|_| admission("block2d inspector field id admission failed"))?
        .try_child(control)
        .map_err(|_| admission("block2d inspector field child admission failed"))?
        .try_build()
        .map_err(|_| admission("block2d inspector field admission failed"))
}

/// 🧾️ One inspector row recorded as DATA — the summary section's window decides how many of these
/// get built, so a field is never materialised for a viewport that will not show it
/// (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
enum InspectorField<'a> {
    /// ✍️ A `blur`-committed text control bound to `patchNodeKind` for `document_field`.
    Text { id: &'a str, label: &'a str, value: &'a str, document_field: &'a str },
    /// 🔒️ A read-only `"{label}: {value}"` row with no control and no binding.
    Readonly { id: &'a str, text: String },
}

fn inspector_row(field: &InspectorField<'_>) -> UiAssemblyResult<BuiltNode> {
    match field {
        InspectorField::Text { id, label, value, document_field } => text_field(id, label, value, document_field),
        InspectorField::Readonly { id, text } => tree_item(*id, text.as_str()),
    }
}

/// 🪟️ The inspector's one summary section is WINDOWED like every other tree container: it stamps its
/// full field count and materialises only the rows its window asks for.
pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let fields = [
        InspectorField::Text { id: "block2d-play-inspector.name", label: labels.name.as_str(), value: &definition.node_kind.name, document_field: "name" },
        InspectorField::Text { id: "block2d-play-inspector.label", label: labels.label.as_str(), value: &definition.node_kind.label, document_field: "label" },
        InspectorField::Text { id: "block2d-play-inspector.variant", label: labels.variant.as_str(), value: definition.node_kind.variant.as_deref().unwrap_or(""), document_field: "variant" },
        InspectorField::Text { id: "block2d-play-inspector.description", label: labels.description.as_str(), value: &definition.node_kind.description, document_field: "description" },
        InspectorField::Readonly { id: "block2d-play-inspector.handle-count", text: format!("{}: {}", labels.handles.as_str(), definition.handles.len()) },
    ];
    PanelTreeBuilder::new("block2d-play-inspector")?
        .window_section(windows, BLOCK2D_INSPECTOR_SUMMARY, Some(ui_label(labels.summary.as_str())?), true, &fields, inspector_row)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
