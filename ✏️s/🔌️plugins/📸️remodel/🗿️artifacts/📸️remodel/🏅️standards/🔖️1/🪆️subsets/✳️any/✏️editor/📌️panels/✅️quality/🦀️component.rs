//! ✅️ Remodel play app panel — the Quality tab: the whole-run QC report, including the watertight
//! sub-report.

use crate::editor::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODEL_PANEL_QC_ID: &str = "remodel.qc";
pub const REMODEL_PLAY_BODY_QC: &str = "remodel.play.qc";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODEL_PANEL_QC_ID.into()), label: LocalizedLabel::native("Quality", "Qualität"), group: PanelGroup::Settings, body_key: Some(REMODEL_PLAY_BODY_QC.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelSnapshot, labels: &RemodelLabels) -> UiNode {
    let Some(qc) = &scene.results.qc else {
        return ui_stack_vertical(vec![ui_text(labels.qc_none)]);
    };
    let mut lines = vec![
        ui_text(Label::data(format!("{}: {:.2}px", labels.qc_reprojection.as_str(), qc.reprojection_rms_px))),
        ui_text(Label::data(format!("{}: {:.1}", labels.qc_track_length.as_str(), qc.mean_track_length))),
        ui_text(Label::data(format!("{}: {:.0}%", labels.qc_registered_ratio.as_str(), qc.registered_frame_ratio * 100.0))),
        ui_text(Label::data(format!("{}: {:.0}%", labels.qc_dense_coverage.as_str(), qc.dense_coverage_ratio * 100.0))),
    ];
    if let Some(rmse) = qc.gcp_checkpoint_rmse {
        lines.push(ui_text(Label::data(format!("{}: {:.3}m", labels.qc_gcp_rmse.as_str(), rmse))));
    }
    if let Some(watertight) = &qc.watertight {
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_watertight.as_str(), watertight.is_watertight))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_boundary_edges.as_str(), watertight.boundary_edge_count))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_components.as_str(), watertight.connected_components))));
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_euler.as_str(), watertight.euler_characteristic))));
        if let Some(genus) = watertight.genus {
            lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_genus.as_str(), genus))));
        }
        lines.push(ui_text(Label::data(format!("{}: {}", labels.qc_closed_fallback.as_str(), watertight.closed_fallback_used))));
    }
    for warning in &qc.warnings {
        lines.push(ui_text(Label::data(format!("⚠️ {warning}"))));
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
    fn a_document_without_a_report_renders_the_empty_state() {
        let mut app = app();
        assert!(render_body(&mut app, REMODEL_PLAY_BODY_QC).contains("No quality report yet"));
    }
}
//#endregion 🧪️Tests
