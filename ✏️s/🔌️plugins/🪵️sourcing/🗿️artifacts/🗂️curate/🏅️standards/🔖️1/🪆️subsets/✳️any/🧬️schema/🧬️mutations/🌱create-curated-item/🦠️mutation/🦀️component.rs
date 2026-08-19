//! 🌱 Sourcing mutation — `CreateCuratedItem`: brings a new id-keyed curated selection into
//! existence.
use crate::artifacts::curate::diff::CurateDiff;
use crate::artifacts::curate::mutations::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, CuratedItem};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-curated-item` payload — full initial payload (`object_id` + starting `count` fixed
/// at creation); a subsequent count adjustment goes through `change-curated-item-count`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-curated-item")]
pub struct CreateCuratedItem {
    #[dsl(block)]
    pub item: CuratedItem,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_curated_item(item: CuratedItem) -> SourcingMutation {
    SourcingMutation::CreateCuratedItem(CreateCuratedItem { item })
}

impl protocol::MutationKind<CurateSnapshot, SourcingMutation> for CreateCuratedItem {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "create", entity: "curated-item", kind: "create-curated-item", record: "CreatedCuratedItem" };

    async fn diff(&self, base: &CurateSnapshot) -> protocol::MutationOutcome<CurateDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CurateSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Curate \"{}\"", self.item.object_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.item.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
