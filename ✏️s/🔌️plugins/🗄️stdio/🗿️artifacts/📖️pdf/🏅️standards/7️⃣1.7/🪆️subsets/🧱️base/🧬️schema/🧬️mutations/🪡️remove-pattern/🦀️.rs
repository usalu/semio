//! 🪡️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-pattern`.

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
pub struct RemovePattern {
    pub id: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemovePattern {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "pattern", kind: "remove-pattern", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_pattern(base, &self.id))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.patterns.iter().find(|item| item.id == self.id).map(|item| PdfMutation::SetPattern(super::set_pattern::SetPattern { pattern: item.clone() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove pattern {}", self.id), &format!("Muster {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
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
