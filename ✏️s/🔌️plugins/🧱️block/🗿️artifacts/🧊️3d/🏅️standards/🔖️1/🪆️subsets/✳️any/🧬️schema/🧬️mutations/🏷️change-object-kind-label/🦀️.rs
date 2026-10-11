//! 🏷️ Block3d mutation — `ChangeObjectKindLabel`: the object kind's `label`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🏷️ `change-object-kind-label` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-object-kind-label")]
pub struct ChangeObjectKindLabel {
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_kind_label(new_label: String) -> Block3dMutation {
    Block3dMutation::ChangeObjectKindLabel(ChangeObjectKindLabel { new_label })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeObjectKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object-kind", kind: "change-object-kind-label", record: "ChangedObjectKindLabel" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change object kind label to \"{}\"", self.new_label), &format!("Objektartbeschriftung auf \"{}\" ändern", self.new_label))
    }
}
//#endregion 🔖️Mutation
