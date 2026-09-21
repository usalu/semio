//! 📐 Puzzle5d mutation — `ScaleTargetVolume`: changes a target volume's freeform pose scale.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 📐 `scale-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "scale-target-volume")]
pub struct ScaleTargetVolume {
    pub id: String,
    pub new_scale: Option<crate::Puzzle5dScale>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_target_volume(id: String, new_scale: Option<crate::Puzzle5dScale>) -> Puzzle5dMutation {
    Puzzle5dMutation::ScaleTargetVolume(ScaleTargetVolume { id, new_scale })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ScaleTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "target-volume", kind: "scale-target-volume", record: "ScaledTargetVolume" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Scale target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
