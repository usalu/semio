//! 🔍️ Puzzle 3d play app panel — the inspector. 🕹️ ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: this used to switch on live selection to show
//! one field group per selected entity kind (object/vortex/attraction/reference/target volume); see
//! `render`'s doc comment for why it now always renders the document summary.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::Puzzle3dScene;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

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
    let rows = vec![
        ui::text(format!("{}: {}", term_labels.schema.as_str(), envelope.fixture.schema)).id("puzzle3d-play-inspector.schema").build(),
        ui::text(format!("{}: {}", term_labels.domain.as_str(), envelope.fixture.domain)).id("puzzle3d-play-inspector.domain").build(),
        ui::text(format!("{}: {}", term_labels.objects.as_str(), envelope.fixture.objects.len())).id("puzzle3d-play-inspector.objects").build(),
    ];
    PanelTreeBuilder::new("puzzle3d-play-inspector")?.section("puzzle3d-play-inspector.empty", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, rows)?.build()
}
//#endregion 🔖️Render
