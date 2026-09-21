//! 📏️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-user-unit`.

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
pub struct SetPageUserUnit {
    pub index: usize,
    pub user_unit: Option<f64>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageUserUnit {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-user-unit", kind: "set-page-user-unit", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_user_unit(self.index, self.user_unit))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).map(|page| PdfMutation::SetPageUserUnit(SetPageUserUnit { index: self.index, user_unit: page.user_unit })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set page {} user unit", self.index), &format!("Seite {} Benutzereinheit setzen", self.index))
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

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
