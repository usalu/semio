//! 🏁️ Remodeling mutation — one atomic reconstruction result: it replaces every reconstruction result
//! lane together and binds raster products into `assets`, naming durable content by id only. The
//! content itself lands earlier in the same edit through `append-content`.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{CameraTrajectory, GeoProducts, QcReportSnapshot, RemodelingMesh, RemodelingSnapshot, SparseCloud};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ One `assets` binding: the asset id bound to complete durable image content, or unbound (`None`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ReconstructionAssetCommit {
    pub id: String,
    pub content_id: Option<String>,
}

/// 🏁️ Atomic result payload: result lanes plus content-id bindings. `mesh: None` keeps the stored mesh.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "commit-reconstruction")]
pub struct CommitReconstruction {
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

/// 🏗️ Builds the single typed result dispatch variant.
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
