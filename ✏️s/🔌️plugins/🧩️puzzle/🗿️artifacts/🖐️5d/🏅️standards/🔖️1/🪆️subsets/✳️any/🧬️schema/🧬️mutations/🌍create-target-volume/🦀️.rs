//! 🌍 Puzzle5d mutation — `CreateTargetVolume`: brings a new id-keyed target volume into existence.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::{Puzzle5dSnapshot, Puzzle5dTargetVolume};

//#region 🔖️Mutation
/// 🌍 `create-target-volume` payload — full initial payload at an optional FINAL-state `index`
/// (`None` appends). A duplicate `target_volume.id` is fatal.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-target-volume")]
pub struct CreateTargetVolume {
    #[dsl(block)]
    pub target_volume: Puzzle5dTargetVolume,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_target_volume(target_volume: Puzzle5dTargetVolume, index: Option<usize>) -> Puzzle5dMutation {
    Puzzle5dMutation::CreateTargetVolume(CreateTargetVolume { target_volume, index })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for CreateTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "target volume", kind: "create-target-volume", record: "CreatedTargetVolume" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create target volume \"{}\"", self.target_volume.id), &format!("Zielvolumen \"{}\" erstellen", self.target_volume.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.target_volume.id.clone()]
    }
}
//#endregion 🔖️Mutation
