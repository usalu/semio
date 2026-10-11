//! 🚀 Puzzle5d mutation — `MoveTargetVolume`: absolute reposition of a target volume's origin.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🚀 `move-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
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
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
