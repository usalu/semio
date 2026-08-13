//! 🧹️ 🧹️ Remodel play app commands command — `clear-dense`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::{replace_dense, replace_geo_products, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory, replace_tracks};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{MeshSource, RemodelMesh, RemodelSnapshot};
use semio_framework_plugin::{mesh_from_kind, ConfigView, ArtifactView, Emit, Fault, MeshData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-dense")]
pub struct ClearDense {}

pub fn handle(_payload: &ClearDense, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_dense(None)]))
}
