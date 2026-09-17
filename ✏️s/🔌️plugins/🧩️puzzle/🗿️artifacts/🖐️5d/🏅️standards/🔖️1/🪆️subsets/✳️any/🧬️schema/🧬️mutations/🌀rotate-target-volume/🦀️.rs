//! 🌀 Puzzle5d mutation — `RotateTargetVolume`: changes a target volume's orientation quaternion.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🌀 `rotate-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-target-volume")]
pub struct RotateTargetVolume {
    pub id: String,
    pub new_orientation: Option<[f64; 4]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_target_volume(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle5dMutation {
    Puzzle5dMutation::RotateTargetVolume(RotateTargetVolume { id, new_orientation })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RotateTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "target-volume", kind: "rotate-target-volume", record: "RotatedTargetVolume" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate target volume \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
