//! 🎯️ Remodeling play app panel — the Calibration tab: per-camera intrinsics, rig extrinsics and ground
//! control points.

use crate::editor::remodeling::terminology::RemodelingLabels;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult};

//#region 🔖️Constants
pub const REMODELING_PANEL_CALIBRATION_ID: &str = "remodeling.calibration";
pub const REMODELING_PLAY_BODY_CALIBRATION: &str = "remodeling.play.calibration";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(REMODELING_PANEL_CALIBRATION_ID.into()),
        label: LocalizedLabel::native("Calibration", "Kalibrierung"),
        group: PanelGroup::Details,
        body_key: Some(REMODELING_PLAY_BODY_CALIBRATION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🪟️ A real photogrammetry rig carries dozens of cameras and ground control points, so both lists
/// are windowed sections; the two count lines stay in their own small fixed summary section so panel
/// chrome never competes with content for one node's slots.
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let summary = crate::editor::remodeling::ui_node_list([
        tree_item("remodeling-calibration.summary", format!("{}: {} - {}: {}", labels.cameras_calibrated.as_str(), scene.calibration.cameras.len(), labels.rig_extrinsics.as_str(), scene.calibration.rig.len())),
        tree_item("remodeling-calibration.gcp-count", format!("{}: {}", labels.gcps.as_str(), scene.gcps.len())),
    ])?;
    PanelTreeBuilder::new("remodeling-calibration")?
        .section("remodeling-calibration.summary", Some(crate::editor::remodeling::ui_label(labels.panel_calibration.as_str())?), true, summary)?
        .window_section(windows, "remodeling-calibration.cameras", Some(crate::editor::remodeling::ui_label(labels.cameras_calibrated.as_str())?), true, &scene.calibration.cameras, |camera| {
            tree_item(format!("remodeling-calibration.camera.{}", camera.id), format!("{} ({}): fx {:.1} fy {:.1}", camera.label, camera.model, camera.fx, camera.fy))
        })?
        .window_section(windows, "remodeling-calibration.gcps", Some(crate::editor::remodeling::ui_label(labels.gcps.as_str())?), true, &scene.gcps, |gcp| {
            tree_item(format!("remodeling-calibration.gcp.{}", gcp.id), format!("{} [{:.2}, {:.2}, {:.2}] ({} obs)", gcp.name, gcp.world_position[0], gcp.world_position[1], gcp.world_position[2], gcp.observations.len()))
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
