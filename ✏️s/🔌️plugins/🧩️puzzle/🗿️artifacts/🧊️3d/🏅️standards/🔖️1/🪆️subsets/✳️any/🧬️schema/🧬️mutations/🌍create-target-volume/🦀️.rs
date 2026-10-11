//! Puzzle3d mutation — `CreateTargetVolume`: brings a new id-keyed target volume into existence.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::{Puzzle3dSnapshot, Puzzle3dTargetVolume};

//#region 🔖️Mutation
/// `create-target-volume` payload — full initial payload at an optional FINAL-state `index` (`None` appends). A
/// duplicate `target_volume.id` is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create target volume \"{}\"", self.target_volume.id), &format!("Zielvolumen \"{}\" erstellen", self.target_volume.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.target_volume.id.clone()]
    }
}
//#endregion 🔖️Mutation
