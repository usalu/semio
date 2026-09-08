//! ✅️ Remodeling play app panel — the Quality tab: the whole-run QC report, including the watertight
//! sub-report.

use crate::RemodelingSnapshot;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult};

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
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiAssemblyResult<BuiltNode> {
    let mut rows: Vec<UiAssemblyResult<BuiltNode>> = Vec::new();
    match &scene.results.qc {
        None => rows.push(tree_item("remodeling-qc.none", labels.qc_none.as_str())),
        Some(qc) => {
            rows.push(tree_item("remodeling-qc.reprojection", format!("{}: {:.2}px", labels.qc_reprojection.as_str(), qc.reprojection_rms_px)));
            rows.push(tree_item("remodeling-qc.track-length", format!("{}: {:.1}", labels.qc_track_length.as_str(), qc.mean_track_length)));
            rows.push(tree_item("remodeling-qc.registered", format!("{}: {:.0}%", labels.qc_registered_ratio.as_str(), qc.registered_frame_ratio * 100.0)));
            rows.push(tree_item("remodeling-qc.dense-coverage", format!("{}: {:.0}%", labels.qc_dense_coverage.as_str(), qc.dense_coverage_ratio * 100.0)));
            if let Some(rmse) = qc.gcp_checkpoint_rmse {
                rows.push(tree_item("remodeling-qc.gcp-rmse", format!("{}: {:.3}m", labels.qc_gcp_rmse.as_str(), rmse)));
            }
            if let Some(watertight) = &qc.watertight {
                rows.push(tree_item("remodeling-qc.watertight", format!("{}: {}", labels.qc_watertight.as_str(), watertight.is_watertight)));
                rows.push(tree_item("remodeling-qc.boundary-edges", format!("{}: {}", labels.qc_boundary_edges.as_str(), watertight.boundary_edge_count)));
                rows.push(tree_item("remodeling-qc.components", format!("{}: {}", labels.qc_components.as_str(), watertight.connected_components)));
                rows.push(tree_item("remodeling-qc.euler", format!("{}: {}", labels.qc_euler.as_str(), watertight.euler_characteristic)));
                if let Some(genus) = watertight.genus {
                    rows.push(tree_item("remodeling-qc.genus", format!("{}: {}", labels.qc_genus.as_str(), genus)));
                }
                rows.push(tree_item("remodeling-qc.closed-fallback", format!("{}: {}", labels.qc_closed_fallback.as_str(), watertight.closed_fallback_used)));
            }
            for (index, warning) in qc.warnings.iter().enumerate() {
                rows.push(tree_item(format!("remodeling-qc.warning.{index}"), format!("⚠️ {warning}")));
            }
        }
    }
    let rows = crate::editor::remodeling::ui_node_list(rows)?;
    PanelTreeBuilder::new("remodeling-qc")?.section("remodeling-qc.report", Some(crate::editor::remodeling::ui_label(labels.panel_qc.as_str())?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn a_document_without_a_report_renders_the_empty_state() {
        let mut app = app().await;
        assert!(render_body(&mut app, REMODELING_PLAY_BODY_QC).await.contains("No quality report yet"));
    }
}
//#endregion 🧪️Tests
