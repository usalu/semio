//! 🔍️ Puzzle 5d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection (grip
//! wins over part wins over fastener) to show one editable field group per resolved entity; see
//! `render`'s doc comment for why it now always renders the document summary.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::Puzzle5dScene;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

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
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> BuiltNode {
    let rows = vec![
        ui::text(format!("{}: {}", labels.schema.as_str(), envelope.document.schema)).id("puzzle5d-play-inspector.schema").build(),
        ui::text(format!("{}: {}", labels.parts.as_str(), envelope.document.parts.len())).id("puzzle5d-play-inspector.parts").build(),
        ui::text(format!("{}: {}", labels.fasteners.as_str(), envelope.document.fasteners.len())).id("puzzle5d-play-inspector.fasteners").build(),
        ui::text(format!("{}: {}", labels.utility.as_str(), envelope.active_utility)).id("puzzle5d-play-inspector.utility").build(),
    ];
    PanelTreeBuilder::new("puzzle5d-play-inspector").section("puzzle5d-play-inspector.empty", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, rows).build()
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
