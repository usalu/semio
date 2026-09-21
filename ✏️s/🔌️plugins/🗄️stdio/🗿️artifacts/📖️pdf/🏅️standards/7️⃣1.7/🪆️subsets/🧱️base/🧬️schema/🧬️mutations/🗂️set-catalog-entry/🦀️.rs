//! 🗂️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-catalog-entry`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCatalogEntry {
    pub key: String,
    pub value: PdfObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetCatalogEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "catalog-entry", kind: "set-catalog-entry", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_catalog_entry(base, &self.key, self.value.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.catalog_extra.iter().find(|entry| entry.key == self.key) { Some(entry) => vec![PdfMutation::SetCatalogEntry(SetCatalogEntry { key: self.key.clone(), value: entry.value.clone() })], None => vec![PdfMutation::RemoveCatalogEntry(super::remove_catalog_entry::RemoveCatalogEntry { key: self.key.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set catalog entry {}", self.key), &format!("Katalogeintrag {} setzen", self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
