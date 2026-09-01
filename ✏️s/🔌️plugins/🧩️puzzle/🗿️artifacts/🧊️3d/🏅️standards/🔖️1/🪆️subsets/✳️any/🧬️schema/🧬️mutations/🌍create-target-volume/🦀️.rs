//! Puzzle3d mutation — `CreateTargetVolume`: brings a new id-keyed target volume into existence.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dSnapshot, Puzzle3dTargetVolume};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `create-target-volume` payload — full initial payload at an optional FINAL-state `index` (`None` appends). A
/// duplicate `target_volume.id` is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-target-volume")]
pub struct CreateTargetVolume {
    #[dsl(block)]
    pub target_volume: Puzzle3dTargetVolume,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_target_volume(target_volume: Puzzle3dTargetVolume, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::CreateTargetVolume(CreateTargetVolume { target_volume, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for CreateTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "target volume", kind: "create-target-volume", record: "CreatedTargetVolume" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create target volume \"{}\"", self.target_volume.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.target_volume.id.clone()]
    }
}
//#endregion 🔖️Mutation
