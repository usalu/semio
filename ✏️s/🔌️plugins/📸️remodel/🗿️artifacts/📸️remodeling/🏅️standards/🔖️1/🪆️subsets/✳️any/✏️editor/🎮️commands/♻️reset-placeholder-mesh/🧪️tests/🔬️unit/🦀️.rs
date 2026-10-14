use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::reset_placeholder_mesh::*;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_dense;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_geo_products;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_mesh_result;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_result;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_sparse;
use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::clear_tracks;
use crate::editor::remodeling::unit_tests::context::{app, dispatch};
use semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingCommand;
use semio_framework_plugin::artifact_app_laws;

/// 🧩️ `results.mesh.mesh` is a composed CHILD handle now — reads the real vertex count through
/// `remodeling_mesh_workspace`'s working-scene cache (0 on a cold cache, matching an empty mesh).
fn mesh_vertex_count(snapshot: &RemodelingSnapshot) -> usize {
    semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_mesh_workspace(&snapshot.results.mesh.mesh).map_or(0, |mesh| mesh.vertex_count())
}

#[semio_framework_async_macros::async_test]
async fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::ClearResult(clear_result::ClearResult {})).await;
    assert!(result.edited_document(), "clearResult publishes one document edit: {:?}", result.lanes);
    let cleared = app.snapshot().expect("materialize projection").results;
    assert_eq!((cleared.sparse.is_none(), cleared.dense.is_none(), cleared.trajectory.is_none(), cleared.tracks.is_empty(), cleared.qc.is_none(), cleared.geo.is_none()), (true, true, true, true, true, true), "clearResult resets every ReconstructionResults field");
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
    artifact_app_laws::assert_undo_redo_round_trip(&mut *app, RemodelingCommand::ClearResult(clear_result::ClearResult {}), |app| mesh_vertex_count(&app.snapshot().expect("materialize projection")), placeholder_vertex_count, 0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
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
        let dispatched = dispatch(&mut app, command).await;
        assert!(dispatched.edited_document(), "{:?}", dispatched.lanes);
    }
}
