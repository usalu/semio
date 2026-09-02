//! 🔍️ Puzzle 5d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection (grip
//! wins over part wins over fastener) to show one editable field group per resolved entity; see
//! `render`'s doc comment for why it now always renders the document summary.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{ui_label, ui_node_list, Puzzle5dScene};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is now
/// framework-owned (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`)
/// and `ArtifactApp::render` never gained that parameter, so this panel has no live selection to
/// render against and always falls through to the document summary below. Flagged to the coordinator
/// as the same framework-level gap noted on `puzzle5d_brush_target_grip` — not fixed here (framework
/// file, out of this crate's remit).
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc("puzzle5d-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(envelope.document.schema.clone())),
        tree_item_desc("puzzle5d-play-inspector.parts", ui_label(labels.parts.as_str())?, Some(envelope.document.parts.len().to_string())),
        tree_item_desc("puzzle5d-play-inspector.fasteners", ui_label(labels.fasteners.as_str())?, Some(envelope.document.fasteners.len().to_string())),
        tree_item_desc("puzzle5d-play-inspector.utility", ui_label(labels.utility.as_str())?, Some(envelope.active_utility.clone())),
    ])?;
    PanelTreeBuilder::new("puzzle5d-play-inspector")?.section("puzzle5d-play-inspector.empty", Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[test]
    fn empty_selection_renders_the_document_summary() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.empty"));
    }
}
//#endregion 🧪️Tests
