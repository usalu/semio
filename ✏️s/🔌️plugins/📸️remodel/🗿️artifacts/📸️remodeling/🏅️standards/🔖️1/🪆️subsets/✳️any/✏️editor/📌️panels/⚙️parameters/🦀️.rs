//! ⚙️ Remodeling play app panel — the Parameters tab: a read-only dump of the 8 param sub-groups (editing
//! happens via the per-group `setXParams` command-palette actions' typed arg forms, not inline fields).

use crate::editor::remodeling::terminology::RemodelingLabels;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult};

//#region 🔖️Constants
pub const REMODELING_PANEL_PARAMETERS_ID: &str = "remodeling.parameters";
pub const REMODELING_PLAY_BODY_PARAMETERS: &str = "remodeling.play.parameters";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(REMODELING_PANEL_PARAMETERS_ID.into()),
        label: LocalizedLabel::native("Parameters", "Parameter"),
        group: PanelGroup::Details,
        body_key: Some(REMODELING_PLAY_BODY_PARAMETERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🪟️ One windowed section — the eight groups are a closed set today, but the section still reports
/// its own `total` so the host paints one uniform scrollbar across every remodeling panel.
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let p = &scene.params;
    let rows: Vec<(&str, String)> = vec![
        (
            "remodeling-parameters.ingest",
            format!(
                "{}: {} {}, {} {}, {} {}px, min sharpness {:.2}",
                labels.params_ingest.as_str(),
                labels.stride_short.as_str(),
                p.ingest.frame_sample_stride,
                labels.max_short.as_str(),
                p.ingest.max_frames,
                labels.downscale_short.as_str(),
                p.ingest.downscale_long_edge_px,
                p.ingest.min_sharpness
            ),
        ),
        ("remodeling-parameters.feature", format!("{}: {:?}, {} {}, {} {}", labels.params_feature.as_str(), p.feature.detector, labels.target_short.as_str(), p.feature.target_count, labels.octaves_short.as_str(), p.feature.octaves)),
        (
            "remodeling-parameters.matching",
            format!("{}: {:?}, {} {:.2}, {} {}", labels.params_matching.as_str(), p.matching.matcher, labels.ratio_short.as_str(), p.matching.ratio_test, labels.window_short.as_str(), p.matching.sequential_window),
        ),
        (
            "remodeling-parameters.sfm",
            format!("{}: {} {}, {} {}, {} {}", labels.params_sfm.as_str(), labels.ransac_short.as_str(), p.sfm.ransac_iterations, labels.min_track_short.as_str(), p.sfm.min_track_length, labels.ba_short.as_str(), p.sfm.ba_max_iterations),
        ),
        ("remodeling-parameters.dense", format!("{}: {:?}, {} {}px", labels.params_dense.as_str(), p.dense.resolution, labels.window_short.as_str(), p.dense.window_radius_px)),
        (
            "remodeling-parameters.mesh",
            format!("{}: {} {:.1}mm, {} {}, watertight {}", labels.params_mesh.as_str(), labels.voxel_short.as_str(), p.mesh.tsdf_voxel_size_mm, labels.target_short.as_str(), p.mesh.decimate_target_triangles, p.mesh.guarantee_watertight),
        ),
        ("remodeling-parameters.motion", format!("{}: {}", labels.params_motion.as_str(), if p.motion.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() })),
        ("remodeling-parameters.geo", format!("{}: {}", labels.params_geo.as_str(), if p.geo.enabled { labels.enabled.as_str() } else { labels.disabled.as_str() })),
    ];
    PanelTreeBuilder::new("remodeling-parameters")?
        .window_section(windows, "remodeling-parameters.groups", Some(crate::editor::remodeling::ui_label(labels.panel_parameters.as_str())?), true, &rows, |(id, text)| tree_item(*id, text.as_str()))?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
