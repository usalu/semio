//! ➕️ Authoritative PDF mutation payload, diff, inverse, and tests for `append-page-content`.

use super::set_page_content::SetPageContent;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendPageContent {
    pub index: usize,
    pub text: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for AppendPageContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "page-content", kind: "append-page-content", record: "Added" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_append_page_content(base, self.index, &self.text))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageContent(SetPageContent { index: self.index, text: page.text.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Append content to page {}", self.index)
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
        assert_eq!(<AppendPageContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "append-page-content");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
