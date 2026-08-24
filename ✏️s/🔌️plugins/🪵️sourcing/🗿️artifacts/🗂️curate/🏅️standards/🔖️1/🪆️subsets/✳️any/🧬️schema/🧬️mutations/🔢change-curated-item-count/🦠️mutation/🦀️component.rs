//! 🔢 Sourcing mutation — `ChangeCuratedItemCount`: sets one curated item's count to a new value.
use crate::artifacts::curate::diff::CurateDiff;
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔢 `change-curated-item-count` payload — addressed by `object_id`; the old count is recovered
/// from `base` at inverse time, never carried on the payload itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-curated-item-count")]
pub struct ChangeCuratedItemCount {
    pub object_id: String,
    pub new_count: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_curated_item_count(object_id: String, new_count: u32) -> SourcingMutation {
    SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id, new_count })
}

impl protocol::MutationKind<CurateSnapshot, SourcingMutation> for ChangeCuratedItemCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "curated-item", kind: "change-curated-item-count", record: "ChangedCuratedItemCount" };

    fn diff(&self, base: &CurateSnapshot) -> protocol::MutationOutcome<CurateDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurateSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set curated count of \"{}\" to {}", self.object_id, self.new_count)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
