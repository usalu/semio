//! 🧩️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-pattern`.

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
pub struct SetPattern {
    pub pattern: PdfPattern,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPattern {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "pattern", kind: "set-pattern", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_pattern(base, self.pattern.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.patterns.iter().find(|item| item.id == self.pattern.id) { Some(previous) => vec![PdfMutation::SetPattern(SetPattern { pattern: previous.clone() })], None => vec![PdfMutation::RemovePattern(super::remove_pattern::RemovePattern { id: self.pattern.id.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set pattern {}", self.pattern.id), &format!("Muster {} setzen", self.pattern.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.pattern.id.clone()]
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
