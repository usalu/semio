//! 🧵️ Remodeling play app panel — the Results tab: the products a run (partially) produced.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult};

//#region 🔖️Constants
pub const REMODELING_PANEL_RESULTS_ID: &str = "remodeling.results";
pub const REMODELING_PLAY_BODY_RESULTS: &str = "remodeling.play.results";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_RESULTS_ID.into()), label: LocalizedLabel::native("Results", "Ergebnisse"), group: PanelGroup::Workbench, body_key: Some(REMODELING_PLAY_BODY_RESULTS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiAssemblyResult<BuiltNode> {
    let results = &scene.results;
    // 🧩️ The composed handle resolves only fixed constants or committed bounded reconstruction
    // content; unavailable content reports 0/0 rather than fabricating a count.
    let mesh = crate::artifacts::remodeling::resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &results.mesh.mesh).unwrap_or_default();
    let mesh_label = format!("{}: {:?}, {} {}, {} {}", labels.mesh.as_str(), results.mesh.source, mesh.vertex_count(), labels.vertices.as_str(), mesh.triangle_count(), labels.triangles.as_str());
    let sparse_label = results
        .sparse
        .as_ref()
        .map_or_else(|| format!("{}: {}", labels.sparse_cloud.as_str(), labels.results_none.as_str()), |sparse| format!("{}: {}", labels.sparse_cloud.as_str(), sparse.points.to_f32_vec_from(&scene.durable_artifacts).len() / 3));
    let dense_label = results.dense.as_ref().map_or_else(|| format!("{}: {}", labels.dense_cloud.as_str(), labels.results_none.as_str()), |dense| format!("{}: {}", labels.dense_cloud.as_str(), dense.positions.to_f32_vec().len() / 3));
    let trajectory_label =
        results.trajectory.as_ref().map_or_else(|| format!("{}: {}", labels.trajectory.as_str(), labels.results_none.as_str()), |trajectory| format!("{}: {} {}", labels.trajectory.as_str(), trajectory.poses.len(), labels.poses.as_str()));
    let geo_label = results.geo.as_ref().map_or_else(|| format!("{}: {}", labels.geo_products.as_str(), labels.results_none.as_str()), |_| format!("{}: {}", labels.geo_products.as_str(), labels.available.as_str()));
    let rows = crate::editor::remodeling::ui_node_list([
        tree_item("remodeling-results.mesh", mesh_label),
        tree_item("remodeling-results.sparse", sparse_label),
        tree_item("remodeling-results.dense", dense_label),
        tree_item("remodeling-results.trajectory", trajectory_label),
        tree_item("remodeling-results.geo", geo_label),
    ])?;
    PanelTreeBuilder::new("remodeling-results")?.section("remodeling-results.products", Some(crate::editor::remodeling::ui_label(labels.panel_results.as_str())?), true, rows)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn a_fresh_document_reports_no_sparse_dense_trajectory_or_geo_products() {
        let mut app = app().await;
        let body = render_body(&mut app, REMODELING_PLAY_BODY_RESULTS).await;
        assert_eq!(body.matches("none").count(), 4, "sparse/dense/trajectory/geo all report 'none': {body}");
    }
}
//#endregion 🧪️Tests
