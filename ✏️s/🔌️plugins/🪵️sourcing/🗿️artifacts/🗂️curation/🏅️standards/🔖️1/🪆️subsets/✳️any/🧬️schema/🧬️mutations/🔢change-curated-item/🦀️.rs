//! 🔢 Direct `change-curated-item-count` mutation owner: sets one curated item's count.
use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Mutation
/// 🔢 `change-curated-item-count` payload — addressed by `object_id`; the old count is recovered
/// from `base` at inverse time, never carried on the payload itself.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-curated-item-count")]
pub struct ChangeCuratedItemCount {
    pub object_id: String,
    pub new_count: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_curated_item_count(object_id: String, new_count: u32) -> SourcingMutation {
    SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id, new_count })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for ChangeCuratedItemCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "curated-item", kind: "change-curated-item-count", record: "ChangedCuratedItemCount" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Result<Vec<SourcingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set curated count of \"{}\" to {}", self.object_id, self.new_count), &format!("Kuratierte Anzahl von \"{}\" auf {} setzen", self.object_id, self.new_count))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
