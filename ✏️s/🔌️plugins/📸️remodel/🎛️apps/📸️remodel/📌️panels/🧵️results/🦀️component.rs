//! 🧵️ Remodel play app panel — the Results tab: the products a run (partially) produced.

use crate::apps::remodel::terminology::RemodelLabels;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODEL_PANEL_RESULTS_ID: &str = "remodel.results";
pub const REMODEL_PLAY_BODY_RESULTS: &str = "remodel.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODEL_PANEL_RESULTS_ID.into()), label: LocalizedLabel::native("Results", "Ergebnisse"), group: PanelGroup::Workbench, body_key: Some(REMODEL_PLAY_BODY_RESULTS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelSnapshot, labels: &RemodelLabels) -> UiNode {
    let results = &scene.results;
    let mesh_label = format!("{}: {:?}, {} {}, {} {}", labels.mesh.as_str(), results.mesh.source, results.mesh.mesh.vertex_count(), labels.vertices.as_str(), results.mesh.mesh.triangle_count(), labels.triangles.as_str());
    let sparse_label = results.sparse.as_ref().map_or_else(|| format!("{}: {}", labels.sparse_cloud.as_str(), labels.results_none.as_str()), |sparse| format!("{}: {}", labels.sparse_cloud.as_str(), sparse.points.to_f32_vec().len() / 3));
    let dense_label = results.dense.as_ref().map_or_else(|| format!("{}: {}", labels.dense_cloud.as_str(), labels.results_none.as_str()), |dense| format!("{}: {}", labels.dense_cloud.as_str(), dense.positions.to_f32_vec().len() / 3));
    let trajectory_label =
        results.trajectory.as_ref().map_or_else(|| format!("{}: {}", labels.trajectory.as_str(), labels.results_none.as_str()), |trajectory| format!("{}: {} {}", labels.trajectory.as_str(), trajectory.poses.len(), labels.poses.as_str()));
    let geo_label = results.geo.as_ref().map_or_else(|| format!("{}: {}", labels.geo_products.as_str(), labels.results_none.as_str()), |_| format!("{}: {}", labels.geo_products.as_str(), labels.available.as_str()));
    ui_stack_vertical(vec![ui_text(Label::data(mesh_label)), ui_text(Label::data(sparse_label)), ui_text(Label::data(dense_label)), ui_text(Label::data(trajectory_label)), ui_text(Label::data(geo_label))])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, render as render_body};

    #[test]
    fn a_fresh_document_reports_no_sparse_dense_trajectory_or_geo_products() {
        let mut app = app();
        let body = render_body(&mut app, REMODEL_PLAY_BODY_RESULTS);
        assert_eq!(body.matches("none").count(), 4, "sparse/dense/trajectory/geo all report 'none': {body}");
    }
}
//#endregion 🧪️Tests
