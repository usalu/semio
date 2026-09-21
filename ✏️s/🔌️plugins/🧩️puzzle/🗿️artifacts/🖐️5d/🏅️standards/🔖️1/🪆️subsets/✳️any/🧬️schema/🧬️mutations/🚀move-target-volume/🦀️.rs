//! 🚀 Puzzle5d mutation — `MoveTargetVolume`: absolute reposition of a target volume's origin.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🚀 `move-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "move-target-volume")]
pub struct MoveTargetVolume {
    pub id: String,
    pub new_origin: [f64; 3],
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_target_volume(id: String, new_origin: [f64; 3]) -> Puzzle5dMutation {
    Puzzle5dMutation::MoveTargetVolume(MoveTargetVolume { id, new_origin })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for MoveTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "target-volume", kind: "move-target-volume", record: "MovedTargetVolume" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Move target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
