//! 🔍️ Puzzle 5d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection (grip
//! wins over part wins over fastener) to show one editable field group per resolved entity; see
//! `render`'s doc comment for why it now always renders the document summary.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::Puzzle5dScene;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
pub async fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "puzzle5d-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("{}: {}", labels.schema.as_str(), envelope.document.schema))),
            ui_text(Label::data(format!("{}: {}", labels.parts.as_str(), envelope.document.parts.len()))),
            ui_text(Label::data(format!("{}: {}", labels.fasteners.as_str(), envelope.document.fasteners.len()))),
            ui_text(Label::data(format!("{}: {}", labels.utility.as_str(), envelope.active_utility))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[test]
    async fn empty_selection_renders_the_document_summary() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.empty"));
    }
}
//#endregion 🧪️Tests
