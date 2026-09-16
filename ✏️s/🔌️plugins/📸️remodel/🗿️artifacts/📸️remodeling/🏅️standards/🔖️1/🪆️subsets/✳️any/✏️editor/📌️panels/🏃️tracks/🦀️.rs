//! 🏃️ Remodeling play app panel — the Tracks tab: moving-object motion tracks. The reconstruction engine
//! does not yet drive the `motion` topic file from `advance()` (its `motion_enabled` flag is accepted
//! but unused), so this stays empty today — a documented gap, not a UI bug.

use crate::editor::remodeling::terminology::RemodelingLabels;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult};

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
/// 🪟️ The status lines are a closed set, the track list is not — two sections, the second windowed.
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let status = if scene.results.tracks.is_empty() {
        crate::editor::remodeling::ui_node_list([tree_item("remodeling-tracks.none", labels.tracks_none.as_str()), tree_item("remodeling-tracks.gap", labels.motion_not_implemented.as_str())])?
    } else {
        crate::editor::remodeling::ui_node_list([tree_item("remodeling-tracks.count", format!("{}: {}", labels.tracks.as_str(), scene.results.tracks.len()))])?
    };
    PanelTreeBuilder::new("remodeling-tracks")?
        .section("remodeling-tracks.motion", Some(crate::editor::remodeling::ui_label(labels.panel_tracks.as_str())?), true, status)?
        .window_section(windows, "remodeling-tracks.tracks", Some(crate::editor::remodeling::ui_label(labels.tracks.as_str())?), true, &scene.results.tracks, |track| {
            tree_item(format!("remodeling-tracks.track.{}", track.id), format!("{} ({:?}): {} frames, {:.2} m/s", track.id, track.class, track.length, track.mean_speed_m_s))
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
