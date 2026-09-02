//! 🧹️ 🧹️ Remodel play app commands command — `clear-mesh-result`.

use crate::artifacts::remodel::mutations::replace_mesh_result;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MeshSource, RemodelMesh, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Results
/// 🫙️ An empty mesh result — what `clearMeshResult`/`clearResult` leave behind.
fn empty_result() -> RemodelMesh {
    RemodelMesh { mesh: crate::artifacts::remodel::empty_remodel_mesh_handle(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
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
#[dsl(keyword = "clear-mesh-result")]
pub struct ClearMeshResult {}

pub async fn handle(_payload: &ClearMeshResult, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_mesh_result(Box::new(empty_result()))]))
}
