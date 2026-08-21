//! 🔍️ Draw play app panel — the inspector (constitutional: was `ui`'s `Panels` region,
//! properties/inspector half).

use crate::artifacts::draw::schema::flatten_draw_layers;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

pub const DRAW_PLAY_BODY_PROPERTIES: &str = "draw.play.properties";

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(DRAW_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-layer field
/// groups (kind/orientation/appearance/layer, patchable via `patchLayers`) this panel used to build
/// from `DrawConfig::selected_ids` are deleted along with that field — selection is framework-owned
/// state now and the editor's `render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `📐️cad`'s `build_properties_panel` object/primitive branches
/// (`🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs`):
/// falls through to the schema/utility/layer-count summary until a resolved-selection render path
/// exists.
pub async fn render(document: &DrawSnapshot, active_utility: &str) -> UiNode {
    ui_stack_vertical(vec![ui_text(Label::data(format!("Schema: {}", DRAW_DOCUMENT_SCHEMA))), ui_text(Label::data(format!("Utility: {active_utility}"))), ui_text(Label::data(format!("Layers: {}", flatten_draw_layers(&document.layers).len())))])
}
//#endregion 🔖️Render
