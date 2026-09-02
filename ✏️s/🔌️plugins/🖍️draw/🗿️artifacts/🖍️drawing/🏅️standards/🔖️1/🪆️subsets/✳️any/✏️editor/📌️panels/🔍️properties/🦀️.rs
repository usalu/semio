//! 🔍️ Drawing play app panel — the inspector (constitutional: was `ui`'s `Panels` region,
//! properties/inspector half).

use crate::artifacts::drawing::schema::flatten_drawing_layers;
use crate::artifacts::drawing::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{built_text_node, BuiltNode, Label, PanelGroup, PanelTabDefinition, PanelTabKind, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

pub const DRAWING_PLAY_BODY_PROPERTIES: &str = "drawing.play.properties";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(DRAWING_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-layer field
/// groups (kind/orientation/appearance/layer, patchable via `patchLayers`) this panel used to build
/// from `DrawingConfig::selected_ids` are deleted along with that field — selection is framework-owned
/// state now and the editor's `render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `📐️cad`'s `build_properties_panel` object/primitive branches
/// (`🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`):
/// falls through to the schema/utility/layer-count summary until a resolved-selection render path
/// exists.
pub fn render(document: &DrawingSnapshot, active_utility: &str) -> UiAssemblyResult<BuiltNode> {
    built_text_node(Label::data(format!("Schema: {DRAWING_DOCUMENT_SCHEMA}; Utility: {active_utility}; Layers: {}", flatten_drawing_layers(&document.layers).len())))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.properties.label", "the fixed Drawing properties summary exceeds its UI label bound"))
}
//#endregion 🔖️Render
