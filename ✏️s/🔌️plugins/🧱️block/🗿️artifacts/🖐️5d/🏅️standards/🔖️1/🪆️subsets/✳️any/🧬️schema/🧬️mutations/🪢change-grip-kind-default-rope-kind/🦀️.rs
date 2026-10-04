//! 🪢 Block5d mutation — `ChangeGripKindDefaultRopeKind`: a grip-kind catalog row's `defaultRopeKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🪢 `change-grip-kind-default-rope-kind` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-grip-kind-default-rope-kind")]
pub struct ChangeGripKindDefaultRopeKind {
    pub id: String,
    pub new_default_rope_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_default_rope_kind(id: String, new_default_rope_kind: String) -> Block5dMutation {
    Block5dMutation::ChangeGripKindDefaultRopeKind(ChangeGripKindDefaultRopeKind { id, new_default_rope_kind })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeGripKindDefaultRopeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-default-rope-kind", record: "ChangedGripKindDefaultRopeKind" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change grip kind \"{}\" default rope kind to \"{}\"", self.id, self.new_default_rope_kind), &format!("Standardseilart von Griffart \"{}\" auf \"{}\" ändern", self.id, self.new_default_rope_kind))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
