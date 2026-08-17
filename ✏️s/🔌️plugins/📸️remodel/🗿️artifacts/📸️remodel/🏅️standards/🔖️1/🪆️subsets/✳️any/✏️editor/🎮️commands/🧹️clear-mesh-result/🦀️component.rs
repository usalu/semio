//! 🧹️ 🧹️ Remodel play app commands command — `clear-mesh-result`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::replace_mesh_result;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MeshSource, RemodelMesh, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, MeshData};
use serde::{Deserialize, Serialize};

//#region 🔖️Results
/// 🫙️ An empty mesh result — what `clearMeshResult`/`clearResult` leave behind.
fn empty_result() -> RemodelMesh {
    RemodelMesh { mesh: crate::artifacts::remodel::mint_and_stash_mesh(MeshData::default()), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-mesh-result")]
pub struct ClearMeshResult {}

pub fn handle(_payload: &ClearMeshResult, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_mesh_result(Box::new(empty_result()))]))
}
