//! 🔍️ Note play app panel — the document-wide properties summary (schema, block count, active
//! utility, snap status).

use crate::artifacts::note::schema::flatten_blocks;
use crate::artifacts::note::NoteSnapshot;
use crate::editor::note::terminology::NotePlayLabels;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_PROPERTIES: &str = "note.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(NOTE_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (a known SDK gap — matches `gis2d`'s inspection panel precedent), so this panel
/// can no longer tell which blocks are selected — it always shows the document-wide summary now; the
/// per-selected-block detail branch (name/x/y/width/height/visible/locked, driven by `patchBlocks`)
/// that used to read `cfg.selected_block_ids` is gone with it.
pub async fn render(document: &NoteSnapshot, active_utility_id: &str, _labels: &NotePlayLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("Schema: {}", document.schema))),
        ui_text(Label::data(format!("Blocks: {}", flatten_blocks(&document.blocks).len()))),
        ui_text(Label::data(format!("Utility: {active_utility_id}"))),
        ui_text(Label::data(format!("Snap: {}", if document.snap_enabled.unwrap_or(false) { format!("{}px", document.snap_grid_spacing.unwrap_or(8.0)) } else { "off".into() }))),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::note::testkit::{note_app, render as render_body};
    use crate::editor::note::NOTE_PLAY_BODY_PROPERTIES as BODY_PROPERTIES;

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = note_app();
        assert!(render_body(&mut app, "note.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_document_wide_summary() {
        let mut app = note_app();
        let json = render_body(&mut app, BODY_PROPERTIES);
        assert!(json.contains("Utility:"));
    }
}
//#endregion 🧪️Tests
