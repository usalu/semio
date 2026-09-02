//! ✏️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-content`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageContent {
    pub index: usize,
    pub text: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-content", kind: "set-page-content", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_content(self.index, &self.text))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageContent(SetPageContent { index: self.index, text: page.text.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Set page {} content", self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<SetPageContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-content");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
