//! 🏃️ Remodeling play app panel — the Tracks tab: moving-object motion tracks. The reconstruction engine
//! does not yet drive the `motion` topic file from `advance()` (its `motion_enabled` flag is accepted
//! but unused), so this stays empty today — a documented gap, not a UI bug.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODELING_PANEL_TRACKS_ID: &str = "remodeling.tracks";
pub const REMODELING_PLAY_BODY_TRACKS: &str = "remodeling.play.tracks";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_TRACKS_ID.into()), label: LocalizedLabel::native("Tracks", "Spuren"), group: PanelGroup::Details, body_key: Some(REMODELING_PLAY_BODY_TRACKS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiNode {
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
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn an_empty_track_list_renders_the_documented_gap_message() {
        let mut app = app();
        assert!(render_body(&mut app, REMODELING_PLAY_BODY_TRACKS).contains("No motion tracks"));
    }
}
//#endregion 🧪️Tests
