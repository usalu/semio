//! 🖥️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-mode`.

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
pub struct SetPageMode {
    pub mode: Option<PdfPageMode>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-mode", kind: "set-page-mode", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_mode(base, self.mode.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        vec![PdfMutation::SetPageMode(SetPageMode { mode: base.page_mode.clone() })]
    }

    fn label(&self) -> String {
        "Set page-mode".to_string()
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
