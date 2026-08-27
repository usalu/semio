//! 🗑️ Direct `delete-curated-item` mutation owner: removes an id-keyed curated selection.
use crate::artifacts::curate::diff::CurateDiff;
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-curated-item` payload — addressed by `object_id` alone; the removed count is
/// recovered from `base` at inverse time, never carried on the payload itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-curated-item")]
pub struct DeleteCuratedItem {
    pub object_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_curated_item(object_id: String) -> SourcingMutation {
    SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id })
}

impl protocol::MutationKind<CurateSnapshot, SourcingMutation> for DeleteCuratedItem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "curated-item", kind: "delete-curated-item", record: "DeletedCuratedItem" };

    fn diff(&self, base: &CurateSnapshot) -> protocol::MutationOutcome<CurateDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurateSnapshot) -> Vec<SourcingMutation> {
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
