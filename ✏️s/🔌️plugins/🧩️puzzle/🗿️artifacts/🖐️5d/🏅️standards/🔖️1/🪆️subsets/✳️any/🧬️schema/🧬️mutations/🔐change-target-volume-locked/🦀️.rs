//! 🔐 Puzzle5d mutation — `ChangeTargetVolumeLocked`: changes a target volume's locked flag.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔐 `change-target-volume-locked` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-target-volume-locked")]
pub struct ChangeTargetVolumeLocked {
    pub id: String,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_target_volume_locked(id: String, new_locked: bool) -> Puzzle5dMutation {
    Puzzle5dMutation::ChangeTargetVolumeLocked(ChangeTargetVolumeLocked { id, new_locked })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangeTargetVolumeLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "target-volume", kind: "change-target-volume-locked", record: "ChangedTargetVolumeLocked" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change target volume locked \"{}\"", self.id), &format!("Sperre von Zielvolumen \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
