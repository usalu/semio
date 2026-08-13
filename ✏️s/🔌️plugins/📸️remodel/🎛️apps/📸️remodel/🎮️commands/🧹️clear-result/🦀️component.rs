//! 🧹️ 🧹️ Remodel play app commands command — `clear-result`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::{replace_dense, replace_geo_products, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory, replace_tracks};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MeshSource, RemodelMesh, RemodelSnapshot};
use semio_framework_plugin::{mesh_from_kind, ConfigView, ArtifactView, Emit, Fault, MeshData};
use serde::{Deserialize, Serialize};

//#region 🔖️Results
/// 📦️ The seeded stand-in mesh a fresh document (and `resetPlaceholderMesh`) carries.
fn placeholder_result() -> RemodelMesh {
    RemodelMesh { mesh: crate::artifacts::remodel::mint_and_stash_mesh(mesh_from_kind("box")), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}

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
#[dsl(keyword = "clear-result")]
pub struct ClearResult {}

/// 🧹️ Resets all seven `ReconstructionResults` fields in one undoable step.
pub fn handle(_payload: &ClearResult, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![
        replace_mesh_result(Box::new(empty_result())),
        replace_sparse(None),
        replace_dense(None),
        replace_trajectory(None),
        replace_tracks(Vec::new()),
        replace_geo_products(None),
        replace_qc(None),
    ]))
}
