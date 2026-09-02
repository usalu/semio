//! 🗑️ Direct `delete-curated-item` mutation owner: removes an id-keyed curated selection.
use crate::artifacts::curation::diff::CurationDiff;
use crate::artifacts::curation::mutations::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;

//#region 🔖️Mutation
/// 🗑️ `delete-curated-item` payload — addressed by `object_id` alone; the removed count is
/// recovered from `base` at inverse time, never carried on the payload itself.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-curated-item")]
pub struct DeleteCuratedItem {
    pub object_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_curated_item(object_id: String) -> SourcingMutation {
    SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for DeleteCuratedItem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "curated-item", kind: "delete-curated-item", record: "DeletedCuratedItem" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove \"{}\" from curation", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
