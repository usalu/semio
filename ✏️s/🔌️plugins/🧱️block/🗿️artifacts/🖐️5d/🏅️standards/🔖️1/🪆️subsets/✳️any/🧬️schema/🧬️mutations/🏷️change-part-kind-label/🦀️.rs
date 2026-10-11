//! 🏷️ Block5d mutation — `ChangePartKindLabel`: the part kind's `label`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🏷️ `change-part-kind-label` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-label")]
pub struct ChangePartKindLabel {
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_label(new_label: String) -> Block5dMutation {
    Block5dMutation::ChangePartKindLabel(ChangePartKindLabel { new_label })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-label", record: "ChangedPartKindLabel" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part kind label to \"{}\"", self.new_label), &format!("Bauteilartbeschriftung auf \"{}\" ändern", self.new_label))
    }
}
//#endregion 🔖️Mutation
