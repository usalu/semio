//! 🌱 Direct `create-curated-item` mutation owner: brings a new id-keyed curated selection into
//! existence.
use crate::artifacts::curation::diff::CurationDiff;
use crate::artifacts::curation::mutations::SourcingMutation;
use crate::artifacts::curation::{CurationSnapshot, CuratedItem};

//#region 🔖️Mutation
/// 🌱 `create-curated-item` payload — full initial payload (`object_id` + starting `count` fixed
/// at creation); a subsequent count adjustment goes through `change-curated-item-count`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-curated-item")]
pub struct CreateCuratedItem {
    #[dsl(block)]
    pub item: CuratedItem,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_curated_item(item: CuratedItem) -> SourcingMutation {
    SourcingMutation::CreateCuratedItem(CreateCuratedItem { item })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for CreateCuratedItem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "curated-item", kind: "create-curated-item", record: "CreatedCuratedItem" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Curation \"{}\"", self.item.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.item.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
