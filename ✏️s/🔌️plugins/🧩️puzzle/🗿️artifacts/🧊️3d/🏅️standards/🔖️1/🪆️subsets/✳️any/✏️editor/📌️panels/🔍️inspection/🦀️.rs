//! 🔍️ Puzzle 3d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection to show
//! one field group per selected entity kind (object/vortex/attraction/reference/target volume); see
//! `render`'s doc comment for why it now always renders the document summary.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{ui_label, ui_node_list, Puzzle3dScene};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.inspector";
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to switch on
/// `envelope.runtime.selection` to show one field group per selected entity kind. Selection is now
/// framework-owned (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`)
/// and `ArtifactApp::render` never gained that parameter, so this panel has no live selection to
/// render against and always falls through to the document summary below. Flagged to the coordinator
/// as the same framework-level gap noted on `puzzle3d_brush_target_vortex` — not fixed here
/// (framework file, out of this crate's remit).
pub fn render(envelope: &Puzzle3dScene, term_labels: &Puzzle3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc("puzzle3d-play-inspector.schema", ui_label(term_labels.schema.as_str())?, Some(envelope.fixture.schema.clone())),
        tree_item_desc("puzzle3d-play-inspector.domain", ui_label(term_labels.domain.as_str())?, Some(envelope.fixture.domain.clone())),
        tree_item_desc("puzzle3d-play-inspector.objects", ui_label(term_labels.objects.as_str())?, Some(envelope.fixture.objects.len().to_string())),
    ])?;
    PanelTreeBuilder::new("puzzle3d-play-inspector")?.section("puzzle3d-play-inspector.empty", Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}
//#endregion 🔖️Render
