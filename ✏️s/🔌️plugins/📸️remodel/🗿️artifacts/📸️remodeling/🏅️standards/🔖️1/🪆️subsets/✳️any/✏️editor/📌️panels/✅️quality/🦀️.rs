//! ✅️ Remodeling play app panel — the Quality tab: the whole-run QC report, including the watertight
//! sub-report.

use crate::editor::remodeling::terminology::RemodelingLabels;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult};

//#region 🔖️Constants
pub const REMODELING_PANEL_QC_ID: &str = "remodeling.qc";
pub const REMODELING_PLAY_BODY_QC: &str = "remodeling.play.qc";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_QC_ID.into()), label: LocalizedLabel::native("Quality", "Qualität"), group: PanelGroup::Settings, body_key: Some(REMODELING_PLAY_BODY_QC.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🪟️ The metric rows are a closed set and the warning list is not, so they live in two windowed
/// sections: a long warning list can never squeeze the metrics out of the report.
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut rows: Vec<(String, String)> = Vec::new();
    let mut warnings: &[String] = &[];
    match &scene.results.qc {
        None => rows.push(("remodeling-qc.none".to_string(), labels.qc_none.as_str().to_string())),
        Some(qc) => {
            rows.push(("remodeling-qc.reprojection".to_string(), format!("{}: {:.2}px", labels.qc_reprojection.as_str(), qc.reprojection_rms_px)));
            rows.push(("remodeling-qc.track-length".to_string(), format!("{}: {:.1}", labels.qc_track_length.as_str(), qc.mean_track_length)));
            rows.push(("remodeling-qc.registered".to_string(), format!("{}: {:.0}%", labels.qc_registered_ratio.as_str(), qc.registered_frame_ratio * 100.0)));
            rows.push(("remodeling-qc.dense-coverage".to_string(), format!("{}: {:.0}%", labels.qc_dense_coverage.as_str(), qc.dense_coverage_ratio * 100.0)));
            if let Some(rmse) = qc.gcp_checkpoint_rmse {
                rows.push(("remodeling-qc.gcp-rmse".to_string(), format!("{}: {:.3}m", labels.qc_gcp_rmse.as_str(), rmse)));
            }
            if let Some(watertight) = &qc.watertight {
                rows.push(("remodeling-qc.watertight".to_string(), format!("{}: {}", labels.qc_watertight.as_str(), watertight.is_watertight)));
                rows.push(("remodeling-qc.boundary-edges".to_string(), format!("{}: {}", labels.qc_boundary_edges.as_str(), watertight.boundary_edge_count)));
                rows.push(("remodeling-qc.components".to_string(), format!("{}: {}", labels.qc_components.as_str(), watertight.connected_components)));
                rows.push(("remodeling-qc.euler".to_string(), format!("{}: {}", labels.qc_euler.as_str(), watertight.euler_characteristic)));
                if let Some(genus) = watertight.genus {
                    rows.push(("remodeling-qc.genus".to_string(), format!("{}: {}", labels.qc_genus.as_str(), genus)));
                }
                rows.push(("remodeling-qc.closed-fallback".to_string(), format!("{}: {}", labels.qc_closed_fallback.as_str(), watertight.closed_fallback_used)));
            }
            warnings = &qc.warnings;
        }
    }
    let indexed: Vec<_> = warnings.iter().enumerate().collect();
    PanelTreeBuilder::new("remodeling-qc")?
        .window_section(windows, "remodeling-qc.report", Some(crate::editor::remodeling::ui_label(labels.panel_qc.as_str())?), true, &rows, |(id, text)| tree_item(id.as_str(), text.as_str()))?
        .window_section(windows, "remodeling-qc.warnings", Some(crate::editor::remodeling::ui_label(labels.panel_qc.as_str())?), true, &indexed, |(index, warning)| {
            tree_item(format!("remodeling-qc.warning.{index}"), format!("⚠️ {warning}"))
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
