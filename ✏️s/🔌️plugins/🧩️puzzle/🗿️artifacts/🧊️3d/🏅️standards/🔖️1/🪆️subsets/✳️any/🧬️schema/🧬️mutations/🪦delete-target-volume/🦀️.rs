//! Puzzle3d mutation — `DeleteTargetVolume`: removes an id-keyed target volume.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// `delete-target-volume` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "delete-target-volume")]
pub struct DeleteTargetVolume {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_target_volume(id: String) -> Puzzle3dMutation {
    Puzzle3dMutation::DeleteTargetVolume(DeleteTargetVolume { id })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DeleteTargetVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "target volume", kind: "delete-target-volume", record: "DeletedTargetVolume" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete target volume \"{}\"", self.id), &format!("Zielvolumen \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
