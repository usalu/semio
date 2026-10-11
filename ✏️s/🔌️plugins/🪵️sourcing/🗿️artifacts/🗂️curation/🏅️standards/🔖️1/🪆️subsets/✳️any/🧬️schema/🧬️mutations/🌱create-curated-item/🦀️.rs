//! 🌱 Direct `create-curated-item` mutation owner: brings a new id-keyed curated selection into
//! existence.
use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::{CurationSnapshot, CuratedItem};

//#region 🔖️Mutation
/// 🌱 `create-curated-item` payload — full initial payload (`object_id` + starting `count` fixed
/// at creation); a subsequent count adjustment goes through `change-curated-item-count`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-curated-item")]
pub struct CreateCuratedItem {
    #[dsl(block)]
    pub item: CuratedItem,
    #[value(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

/// 🏗️ Builder — appends the item after the existing ones.
pub fn create_curated_item(item: CuratedItem) -> SourcingMutation {
    SourcingMutation::CreateCuratedItem(CreateCuratedItem { item, index: None })
}

/// 📍️ Builder — inserts the item at `index` (an index past the end appends).
pub fn create_curated_item_at(item: CuratedItem, index: u32) -> SourcingMutation {
    SourcingMutation::CreateCuratedItem(CreateCuratedItem { item, index: Some(index) })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for CreateCuratedItem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "curated-item", kind: "create-curated-item", record: "CreatedCuratedItem" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Result<Vec<SourcingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Curation \"{}\"", self.item.object_id), &format!("Kuratierung \"{}\"", self.item.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.item.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
