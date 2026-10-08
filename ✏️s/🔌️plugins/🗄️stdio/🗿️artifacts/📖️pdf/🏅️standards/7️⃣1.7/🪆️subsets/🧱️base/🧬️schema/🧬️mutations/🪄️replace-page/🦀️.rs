//! 🪄️ Authoritative PDF mutation payload, diff, inverse, and tests for `replace-page`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::{PdfPage, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplacePage {
    pub index: usize,
    pub page: PdfPage,
}

impl MutationKind<PdfSnapshot, PdfMutation> for ReplacePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "page", kind: "replace-page", record: "Replace" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if self.index >= base.pages.len() {
            return MutationOutcome::error("mutation.target-missing", format!("Page {} does not exist.", self.index), [self.index.to_string()]);
        }
        MutationOutcome::new(diff::diff_replace_page(base, self.index, &self.page))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
        Ok(base.pages.get(self.index).map(|page| PdfMutation::ReplacePage(ReplacePage { index: self.index, page: page.clone() })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace page {}", self.index), &format!("Seite {} ersetzen", self.index))
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

