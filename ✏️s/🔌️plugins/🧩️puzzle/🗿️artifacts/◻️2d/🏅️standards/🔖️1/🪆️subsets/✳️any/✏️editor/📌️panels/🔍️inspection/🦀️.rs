//! 🔍️ Puzzle 2d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection to show
//! per-node id/kind/x/y editable fields; see `render`'s doc comment for why it now always renders the
//! document summary.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, puzzle_extension_id, ui_label, ui_node_list, Puzzle2dScene, PUZZLE2D_FIXTURE_SCHEMA};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc("puzzle2d-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(PUZZLE2D_FIXTURE_SCHEMA.into())),
        tree_item_desc("puzzle2d-play-inspector.extension", ui_label(labels.extension.as_str())?, Some(puzzle_extension_id().into())),
        tree_item_desc("puzzle2d-play-inspector.nodes", ui_label(labels.nodes.as_str())?, Some(fixture_nodes(&envelope.fixture).len().to_string())),
        tree_item_desc("puzzle2d-play-inspector.edges", ui_label(labels.edges.as_str())?, Some(fixture_edges(&envelope.fixture).len().to_string())),
    ])?;
    PanelTreeBuilder::new("puzzle2d-play-inspector")?.section("puzzle2d-play-inspector.summary", Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}
//#endregion 🔖️Render
