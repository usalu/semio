//! 📐️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-media-box`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPageMediaBox {
    pub index: usize,
    pub media_box: [f64; 4],
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageMediaBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-media-box", kind: "set-page-media-box", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_media_box(self.index, self.media_box))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageMediaBox(SetPageMediaBox { index: self.index, media_box: page.media_box })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Set page {} media box", self.index)
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
        assert_eq!(<SetPageMediaBox as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-media-box");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
