//! 🗑️ Direct `delete-curated-item` mutation owner: removes an id-keyed curated selection.
use crate::diff::CurationDiff;
use crate::CurationSnapshot;

//#region 🔖️Mutation
/// 🗑️ `delete-curated-item` payload — addressed by `object_id` alone; the removed count is
/// recovered from `base` at inverse time, never carried on the payload itself.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-curated-item")]
pub struct DeleteCuratedItem {
    pub object_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_curated_item<Op>(object_id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_curated_item::DeleteCuratedItem>,
{
    Op::from(DeleteCuratedItem { object_id })
}

impl<Op> protocol::MutationKind<CurationSnapshot, Op> for DeleteCuratedItem
where
    Op: protocol::Mutation<CurationSnapshot, Diff = CurationDiff> + From<DeleteCuratedItem> + From<crate::standards::v1::subsets::any::schema::mutations::create_curated_item::CreateCuratedItem>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "curated-item", kind: "delete-curated-item", record: "DeletedCuratedItem" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove \"{}\" from curation", self.object_id), &format!("\"{}\" aus Kuratierung entfernen", self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<CurationSnapshot, Op> for DeleteCuratedItem
where
    Op: protocol::ColdMutation<CurationSnapshot, ReferenceDiff = CurationDiff> + From<DeleteCuratedItem> + From<crate::standards::v1::subsets::any::schema::mutations::create_curated_item::CreateCuratedItem>,
{
    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
