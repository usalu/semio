//! 🧺️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-catalog-entry`.

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
pub struct RemoveCatalogEntry {
    pub key: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveCatalogEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "catalog-entry", kind: "remove-catalog-entry", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_catalog_entry(base, &self.key))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.catalog_extra.iter().find(|entry| entry.key == self.key).map(|entry| PdfMutation::SetCatalogEntry(super::set_catalog_entry::SetCatalogEntry { key: self.key.clone(), value: entry.value.clone() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove catalog entry {}", self.key), &format!("Katalogeintrag {} entfernen", self.key))
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
