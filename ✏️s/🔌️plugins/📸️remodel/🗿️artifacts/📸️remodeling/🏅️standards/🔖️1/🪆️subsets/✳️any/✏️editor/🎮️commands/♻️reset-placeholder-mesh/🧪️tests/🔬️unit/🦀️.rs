use super::*;
use crate::editor::remodeling::commands::{clear_dense, clear_geo_products, clear_mesh_result, clear_result, clear_sparse, clear_tracks};
use crate::editor::remodeling::unit_tests::context::{app, dispatch};
use crate::editor::remodeling::RemodelingCommand;
use semio_framework_plugin::artifact_app_laws;

/// 🧩️ `results.mesh.mesh` is a composed CHILD handle now — reads the real vertex count through
/// `remodeling_mesh_workspace`'s working-scene cache (0 on a cold cache, matching an empty mesh).
fn mesh_vertex_count(snapshot: &RemodelingSnapshot) -> usize {
    crate::remodeling_mesh_workspace(&snapshot.results.mesh.mesh).map_or(0, |mesh| mesh.vertex_count())
}

#[semio_framework_async_macros::async_test]
async fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::ClearResult(clear_result::ClearResult {})).await;
    assert_eq!(result.mutations.len(), 7, "clearResult resets all 7 ReconstructionResults fields");
    assert_eq!(mesh_vertex_count(&app.snapshot().expect("materialize projection")), 0);
    dispatch(&mut app, RemodelingCommand::ResetPlaceholderMesh(ResetPlaceholderMesh {})).await;
    assert_eq!(app.snapshot().expect("materialize projection").results.mesh.source, MeshSource::Placeholder);
    assert!(mesh_vertex_count(&app.snapshot().expect("materialize projection")) > 0);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = app().await;
    let placeholder_vertex_count = mesh_vertex_count(&app.snapshot().expect("materialize projection"));
    assert!(placeholder_vertex_count > 0, "the seeded placeholder box must have vertices");
    artifact_app_laws::assert_undo_redo_round_trip(&mut app, RemodelingCommand::ClearResult(clear_result::ClearResult {}), |app| mesh_vertex_count(&app.snapshot().expect("materialize projection")), placeholder_vertex_count, 0).await;
}

#[semio_framework_async_macros::async_test]
async fn each_narrow_clear_touches_exactly_one_result_field() {
    let mut app = app().await;
    for command in [
        RemodelingCommand::ClearSparse(clear_sparse::ClearSparse {}),
        RemodelingCommand::ClearDense(clear_dense::ClearDense {}),
        RemodelingCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
        RemodelingCommand::ClearTracks(clear_tracks::ClearTracks {}),
        RemodelingCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
    ] {
        assert_eq!(dispatch(&mut app, command).await.mutations.len(), 1);
    }
}
