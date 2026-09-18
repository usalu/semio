//! 📖️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-layout`.

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
pub struct SetPageLayout {
    pub layout: Option<PdfPageLayout>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-layout", kind: "set-page-layout", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_layout(base, self.layout.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        vec![PdfMutation::SetPageLayout(SetPageLayout { layout: base.page_layout.clone() })]
    }

    fn label(&self) -> String {
        "Set page-layout".to_string()
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
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
