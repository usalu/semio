//! 📥️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-page`.

use super::remove_page::RemovePage;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::{PdfPage, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPage {
    pub index: usize,
    pub page: PdfPage,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertPage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "Insert" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_insert_page(self.index, self.page.clone()))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfMutation::RemovePage(RemovePage { index: self.index })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert page {}", self.index), &format!("Seite {} einfügen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

