//! 🏃️ Remodel play app panel — the Tracks tab: moving-object motion tracks. The reconstruction engine
//! does not yet drive the `motion` topic file from `advance()` (its `motion_enabled` flag is accepted
//! but unused), so this stays empty today — a documented gap, not a UI bug.

use crate::editor::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODEL_PANEL_TRACKS_ID: &str = "remodel.tracks";
pub const REMODEL_PLAY_BODY_TRACKS: &str = "remodel.play.tracks";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODEL_PANEL_TRACKS_ID.into()), label: LocalizedLabel::native("Tracks", "Spuren"), group: PanelGroup::Details, body_key: Some(REMODEL_PLAY_BODY_TRACKS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelSnapshot, labels: &RemodelLabels) -> UiNode {
    if scene.results.tracks.is_empty() {
        return ui_stack_vertical(vec![ui_text(labels.tracks_none), ui_text(labels.motion_not_implemented)]);
    }
    let mut lines = vec![ui_text(Label::data(format!("{}: {}", labels.tracks.as_str(), scene.results.tracks.len())))];
    for track in &scene.results.tracks {
        lines.push(ui_text(Label::data(format!("{} ({:?}): {} frames, {:.2} m/s", track.id, track.class, track.length, track.mean_speed_m_s))));
    }
    ui_stack_vertical(lines)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodel::testkit::{app, render as render_body};

    #[test]
    fn an_empty_track_list_renders_the_documented_gap_message() {
        let mut app = app();
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_TRACKS).contains("No motion tracks"));
    }
}
//#endregion 🧪️Tests
