//! 🧹️ 🧹️ Remodeling play app commands command — `clear-result`.

use crate::artifacts::remodeling::mutations::{replace_dense, replace_geo_products, replace_mesh_result, replace_qc, replace_sparse, replace_tracks, replace_trajectory};
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{MeshSource, RemodelingMesh, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Results
/// 🫙️ An empty mesh result — what `clearMeshResult`/`clearResult` leave behind.
fn empty_result() -> RemodelingMesh {
    RemodelingMesh { mesh: crate::artifacts::remodeling::empty_remodeling_mesh_handle(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
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
#[dsl(keyword = "clear-result")]
pub struct ClearResult {}

/// 🧹️ Resets all seven `ReconstructionResults` fields in one undoable step.
pub async fn handle(_payload: &ClearResult, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_mesh_result(Box::new(empty_result())), replace_sparse(None), replace_dense(None), replace_trajectory(None), replace_tracks(Vec::new()), replace_geo_products(None), replace_qc(None)]))
}
