//! ⚙️ Remodel play app panel — the Parameters tab: a read-only dump of the 8 param sub-groups (editing
//! happens via the per-group `setXParams` command-palette actions' typed arg forms, not inline fields).

use crate::editor::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODEL_PANEL_PARAMETERS_ID: &str = "remodel.parameters";
pub const REMODEL_PLAY_BODY_PARAMETERS: &str = "remodel.play.parameters";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODEL_PANEL_PARAMETERS_ID.into()), label: LocalizedLabel::native("Parameters", "Parameter"), group: PanelGroup::Details, body_key: Some(REMODEL_PLAY_BODY_PARAMETERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelSnapshot, labels: &RemodelLabels) -> UiNode {
    let p = &scene.params;
    ui_stack_vertical(vec![
        ui_text(Label::data(format!(
            "{}: {} {}, {} {}, {} {}px, min sharpness {:.2}",
            labels.params_ingest.as_str(),
            labels.stride_short.as_str(),
            p.ingest.frame_sample_stride,
            labels.max_short.as_str(),
            p.ingest.max_frames,
            labels.downscale_short.as_str(),
            p.ingest.downscale_long_edge_px,
            p.ingest.min_sharpness
        ))),
        ui_text(Label::data(format!("{}: {:?}, {} {}, {} {}", labels.params_feature.as_str(), p.feature.detector, labels.target_short.as_str(), p.feature.target_count, labels.octaves_short.as_str(), p.feature.octaves))),
        ui_text(Label::data(format!("{}: {:?}, {} {:.2}, {} {}", labels.params_matching.as_str(), p.matching.matcher, labels.ratio_short.as_str(), p.matching.ratio_test, labels.window_short.as_str(), p.matching.sequential_window))),
        ui_text(Label::data(format!(
            "{}: {} {}, {} {}, {} {}",
            labels.params_sfm.as_str(),
            labels.ransac_short.as_str(),
            p.sfm.ransac_iterations,
            labels.min_track_short.as_str(),
            p.sfm.min_track_length,
            labels.ba_short.as_str(),
            p.sfm.ba_max_iterations
        ))),
        ui_text(Label::data(format!("{}: {:?}, {} {}px", labels.params_dense.as_str(), p.dense.resolution, labels.window_short.as_str(), p.dense.window_radius_px))),
        ui_text(Label::data(format!(
            "{}: {} {:.1}mm, {} {}, watertight {}",
            labels.params_mesh.as_str(),
            labels.voxel_short.as_str(),
            p.mesh.tsdf_voxel_size_mm,
            labels.target_short.as_str(),
            p.mesh.decimate_target_triangles,
            p.mesh.guarantee_watertight
        ))),
        ui_text(Label::data(format!("{}: {}", labels.params_motion.as_str(), if p.motion.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() }))),
        ui_text(Label::data(format!("{}: {}", labels.params_geo.as_str(), if p.geo.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() }))),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodel::commands::set_ingest_params::SetIngestParams;
    use crate::editor::remodel::testkit::{app, dispatch, render as render_body};
    use crate::editor::remodel::RemodelCommand;

    #[test]
    fn the_parameters_panel_reflects_a_live_param_edit() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetIngestParams(SetIngestParams { frame_sample_stride: 9, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }));
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_PARAMETERS).contains("stride 9"));
    }
}
//#endregion 🧪️Tests
