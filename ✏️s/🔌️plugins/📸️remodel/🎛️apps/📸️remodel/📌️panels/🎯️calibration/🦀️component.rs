//! 🎯️ Remodel play app panel — the Calibration tab: per-camera intrinsics, rig extrinsics and ground
//! control points.

use crate::apps::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODEL_PANEL_CALIBRATION_ID: &str = "remodel.calibration";
pub const REMODEL_PLAY_BODY_CALIBRATION: &str = "remodel.play.calibration";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(REMODEL_PANEL_CALIBRATION_ID.into()),
        label: LocalizedLabel::native("Calibration", "Kalibrierung"),
        group: PanelGroup::Details,
        body_key: Some(REMODEL_PLAY_BODY_CALIBRATION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelSnapshot, labels: &RemodelLabels) -> UiNode {
    let mut lines = vec![ui_text(Label::data(format!("{}: {} - {}: {}", labels.cameras_calibrated.as_str(), scene.calibration.cameras.len(), labels.rig_extrinsics.as_str(), scene.calibration.rig.len())))];
    for camera in &scene.calibration.cameras {
        lines.push(ui_text(Label::data(format!("{} ({}): fx {:.1} fy {:.1}", camera.label, camera.model, camera.fx, camera.fy))));
    }
    lines.push(ui_text(Label::data(format!("{}: {}", labels.gcps.as_str(), scene.gcps.len()))));
    for gcp in &scene.gcps {
        lines.push(ui_text(Label::data(format!("{} [{:.2}, {:.2}, {:.2}] ({} obs)", gcp.name, gcp.world_position[0], gcp.world_position[1], gcp.world_position[2], gcp.observations.len()))));
    }
    ui_stack_vertical(lines)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::commands::calibration::add_gcp::AddGcp;
    use crate::apps::remodel::testkit::{app, dispatch, render as render_body};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn the_calibration_panel_lists_added_ground_control_points() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::AddGcp(AddGcp { name: "Corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 }));
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_CALIBRATION).contains("Corner"));
    }
}
//#endregion 🧪️Tests
