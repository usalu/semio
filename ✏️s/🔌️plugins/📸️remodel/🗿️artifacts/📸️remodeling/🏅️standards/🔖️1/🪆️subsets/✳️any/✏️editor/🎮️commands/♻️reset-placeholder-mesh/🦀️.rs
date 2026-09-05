//! 🧹️ 🧹️ Remodeling play app commands command — `reset-placeholder-mesh`.

use crate::artifacts::remodeling::mutations::replace_mesh_result;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{MeshSource, RemodelingMesh, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Results
/// 📦️ The seeded stand-in mesh a fresh document (and `resetPlaceholderMesh`) carries.
fn placeholder_result() -> RemodelingMesh {
    RemodelingMesh { mesh: crate::artifacts::remodeling::placeholder_remodeling_mesh_handle(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}
//#endregion 🔖️Results

//#region 🔖️ResetPlaceholderMesh
//#endregion 🔖️ResetPlaceholderMesh

//#region 🔖️ClearSparse
//#endregion 🔖️ClearSparse

//#region 🔖️ClearDense
//#endregion 🔖️ClearDense

//#region 🔖️ClearMeshResult
//#endregion 🔖️ClearMeshResult

//#region 🔖️ClearTracks
//#endregion 🔖️ClearTracks

//#region 🔖️ClearGeoProducts
//#endregion 🔖️ClearGeoProducts

//#region 🔖️ClearResult
//#endregion 🔖️ClearResult

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "reset-placeholder-mesh")]
pub struct ResetPlaceholderMesh {}

pub async fn handle(_payload: &ResetPlaceholderMesh, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_mesh_result(Box::new(placeholder_result()))]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::commands::{clear_dense, clear_geo_products, clear_mesh_result, clear_result, clear_sparse, clear_tracks};
    use crate::editor::remodeling::testkit::{app, dispatch};
    use crate::editor::remodeling::RemodelingCommand;
    use semio_framework_plugin::testkit;

    /// 🧩️ `results.mesh.mesh` is a composed CHILD handle now — reads the real vertex count through
    /// `remodeling_mesh_workspace`'s working-scene cache (0 on a cold cache, matching an empty mesh).
    async fn mesh_vertex_count(snapshot: &RemodelingSnapshot) -> usize {
        crate::artifacts::remodeling::remodeling_mesh_workspace(&snapshot.results.mesh.mesh).map_or(0, |mesh| mesh.vertex_count())
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelingCommand::ClearResult(clear_result::ClearResult {}));
        assert_eq!(result.mutations.len(), 7, "clearResult resets all 7 ReconstructionResults fields");
        assert_eq!(mesh_vertex_count(&app.snapshot().expect("materialize projection")), 0);
        dispatch(&mut app, RemodelingCommand::ResetPlaceholderMesh(ResetPlaceholderMesh {}));
        assert_eq!(app.snapshot().expect("materialize projection").results.mesh.source, MeshSource::Placeholder);
        assert!(mesh_vertex_count(&app.snapshot().expect("materialize projection")) > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = app();
        let placeholder_vertex_count = mesh_vertex_count(&app.snapshot().expect("materialize projection"));
        assert!(placeholder_vertex_count > 0, "the seeded placeholder box must have vertices");
        testkit::assert_undo_redo_round_trip(&mut app, RemodelingCommand::ClearResult(clear_result::ClearResult {}), |app| mesh_vertex_count(&app.snapshot().expect("materialize projection")), placeholder_vertex_count, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn each_narrow_clear_touches_exactly_one_result_field() {
        let mut app = app();
        for command in [
            RemodelingCommand::ClearSparse(clear_sparse::ClearSparse {}),
            RemodelingCommand::ClearDense(clear_dense::ClearDense {}),
            RemodelingCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
            RemodelingCommand::ClearTracks(clear_tracks::ClearTracks {}),
            RemodelingCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
        ] {
            assert_eq!(dispatch(&mut app, command).mutations.len(), 1);
        }
    }
}
//#endregion 🧪️Tests
