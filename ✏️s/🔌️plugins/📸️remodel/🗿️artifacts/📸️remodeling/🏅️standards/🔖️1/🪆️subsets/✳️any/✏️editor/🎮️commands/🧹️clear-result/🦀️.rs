//! 🧹️ 🧹️ Remodeling play app commands command — `clear-result`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::{replace_dense, replace_geo_products, replace_mesh_result, replace_qc, replace_sparse, replace_tracks, replace_trajectory};
use crate::op::RemodelingMutation;
use crate::{MeshSource, RemodelingMesh, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Results
/// 🫙️ An empty mesh result — what `clearMeshResult`/`clearResult` leave behind.
fn empty_result() -> RemodelingMesh {
    RemodelingMesh { mesh: crate::empty_remodeling_mesh_handle(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
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
pub fn handle(_payload: &ClearResult, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_mesh_result(Box::new(empty_result())), replace_sparse(None), replace_dense(None), replace_trajectory(None), replace_tracks(Vec::new()), replace_geo_products(None), replace_qc(None)]))
}
