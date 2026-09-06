//! 🏃️ Remodeling play app panel — the Tracks tab: moving-object motion tracks. The reconstruction engine
//! does not yet drive the `motion` topic file from `advance()` (its `motion_enabled` flag is accepted
//! but unused), so this stays empty today — a documented gap, not a UI bug.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult};

//#region 🔖️Constants
pub const REMODELING_PANEL_TRACKS_ID: &str = "remodeling.tracks";
pub const REMODELING_PLAY_BODY_TRACKS: &str = "remodeling.play.tracks";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_TRACKS_ID.into()), label: LocalizedLabel::native("Tracks", "Spuren"), group: PanelGroup::Details, body_key: Some(REMODELING_PLAY_BODY_TRACKS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiAssemblyResult<BuiltNode> {
    let mut rows: Vec<UiAssemblyResult<BuiltNode>> = Vec::new();
    if scene.results.tracks.is_empty() {
        rows.push(tree_item("remodeling-tracks.none", labels.tracks_none.as_str()));
        rows.push(tree_item("remodeling-tracks.gap", labels.motion_not_implemented.as_str()));
    } else {
        rows.push(tree_item("remodeling-tracks.count", format!("{}: {}", labels.tracks.as_str(), scene.results.tracks.len())));
        for track in &scene.results.tracks {
            rows.push(tree_item(format!("remodeling-tracks.track.{}", track.id), format!("{} ({:?}): {} frames, {:.2} m/s", track.id, track.class, track.length, track.mean_speed_m_s)));
        }
    }
    let rows = crate::editor::remodeling::ui_node_list(rows)?;
    PanelTreeBuilder::new("remodeling-tracks")?.section("remodeling-tracks.motion", Some(crate::editor::remodeling::ui_label(labels.panel_tracks.as_str())?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn an_empty_track_list_renders_the_documented_gap_message() {
        let mut app = app().await;
        assert!(render_body(&mut app, REMODELING_PLAY_BODY_TRACKS).await.contains("No motion tracks"));
    }
}
//#endregion 🧪️Tests
