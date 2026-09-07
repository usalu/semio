//! 🏁️ Remodeling mutation — one atomic durable reconstruction terminal event.

use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{
    CameraTrajectory, GeoProducts, ImageAsset, QcReportSnapshot, ReconstructionJob, RemodelingMesh, RemodelingSnapshot, SparseCloud,
};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ One compact named content handle published by a reconstruction commit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ReconstructionAssetCommit {
    pub id: String,
    #[dsl(block)]
    pub asset: ImageAsset,
}

/// 🏁️ Atomic terminal payload: compact handles plus bounded scalar/report metadata.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "commit-reconstruction")]
pub struct CommitReconstruction {
    #[dsl(block)]
    pub job: ReconstructionJob,
    #[dsl(block)]
    pub sparse: Option<SparseCloud>,
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
    #[dsl(block)]
    pub mesh: Option<Box<RemodelingMesh>>,
    #[dsl(block)]
    pub geo: Option<GeoProducts>,
    #[dsl(block)]
    pub qc: Option<QcReportSnapshot>,
    #[dsl(table)]
    pub assets: Vec<ReconstructionAssetCommit>,
}

/// 🏗️ Builds the single typed terminal dispatch variant.
pub fn commit_reconstruction(payload: CommitReconstruction) -> RemodelingMutation {
    RemodelingMutation::CommitReconstruction(payload)
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for CommitReconstruction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "commit", entity: "reconstruction", kind: "commit-reconstruction", record: "CommittedReconstruction" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Commit reconstruction".into()
    }
}
//#endregion 🔖️Mutation
