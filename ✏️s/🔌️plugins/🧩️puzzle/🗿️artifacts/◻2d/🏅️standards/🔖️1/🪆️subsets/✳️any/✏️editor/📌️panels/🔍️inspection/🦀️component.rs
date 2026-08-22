//! 🔍️ Puzzle 2d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection to show
//! per-node id/kind/x/y editable fields; see `render`'s doc comment for why it now always renders the
//! document summary.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, puzzle_extension_id, Puzzle2dScene, PUZZLE2D_FIXTURE_SCHEMA};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasChildren};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PUZZLE2D_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is now
/// framework-owned (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`)
/// and `ArtifactApp::render` never gained that parameter, so this panel has no live selection to
/// render against and always falls through to the document summary. Flagged to the coordinator as
/// the same framework-level gap noted throughout this crate's other panels — not fixed here
/// (framework file, out of this crate's remit).
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> BuiltNode {
    ui::column()
        .children(vec![
            ui::text(format!("{}: {PUZZLE2D_FIXTURE_SCHEMA}", labels.schema.as_str())).build(),
            ui::text(format!("{}: {}", labels.extension.as_str(), puzzle_extension_id())).build(),
            ui::text(format!("{}: {}", labels.nodes.as_str(), fixture_nodes(&envelope.fixture).len())).build(),
            ui::text(format!("{}: {}", labels.edges.as_str(), fixture_edges(&envelope.fixture).len())).build(),
        ])
        .build()
}
//#endregion 🔖️Render
