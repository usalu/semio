//! 🗂️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-catalog-entry`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCatalogEntry {
    pub key: String,
    pub value: PdfObject,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetCatalogEntry
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetCatalogEntry> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::RemoveCatalogEntry>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "catalog-entry", kind: "set-catalog-entry", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set catalog entry {}", self.key), &format!("Katalogeintrag {} setzen", self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetCatalogEntry
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetCatalogEntry> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::RemoveCatalogEntry>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_catalog_entry(base, &self.key, self.value.clone(), self.index))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.catalog_extra.iter().find(|entry| entry.key == self.key) { Some(entry) => vec![Op::from(SetCatalogEntry { key: self.key.clone(), value: entry.value.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::RemoveCatalogEntry { key: self.key.clone() })] }
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

