//! 🏁️ Remodel mutation — one atomic durable reconstruction terminal event.

use crate::artifacts::remodel::{CameraTrajectory, GeoProducts, ImageAsset, QcReportSnapshot, ReconstructionJob, RemodelMesh, RemodelSnapshot, SparseCloud, committed_remodel_asset_handle, durable_staged_remodel_asset, durable_staged_remodel_mesh};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::{RemodelMutation, replace_geo_products, replace_job, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ One compact named content handle published by a reconstruction commit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ReconstructionAssetCommit {
    pub id: String,
    #[dsl(block)]
    pub asset: ImageAsset,
}

/// 🏁️ Atomic terminal payload: compact handles plus bounded scalar/report metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "commit-reconstruction")]
pub struct CommitReconstruction {
    #[dsl(block)]
    pub job: ReconstructionJob,
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(block)]
    pub mesh: Option<Box<RemodelMesh>>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
    #[dsl(table)]
    pub assets: Vec<ReconstructionAssetCommit>,
}

/// 🏗️ Builds the single typed terminal dispatch variant.
pub fn commit_reconstruction(payload: CommitReconstruction) -> RemodelMutation {
    RemodelMutation::CommitReconstruction(payload)
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for CommitReconstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "commit", entity: "reconstruction", kind: "commit-reconstruction", record: "CommittedReconstruction" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        "Commit reconstruction".into()
    }
}
//#endregion 🔖️Mutation
