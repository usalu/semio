//! 🙈 Puzzle5d mutation — `ChangeTargetVolumeHidden`: changes a target volume's hidden flag.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🙈 `change-target-volume-hidden` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-target-volume-hidden")]
pub struct ChangeTargetVolumeHidden {
    pub id: String,
    pub new_hidden: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_target_volume_hidden(id: String, new_hidden: bool) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeTargetVolumeHidden(ChangeTargetVolumeHidden { id, new_hidden })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeTargetVolumeHidden {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "target-volume", kind: "change-target-volume-hidden", record: "ChangedTargetVolumeHidden" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change target volume hidden \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
