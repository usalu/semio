//! 🔤️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-font`.

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
pub struct SetFont {
    pub font: PdfFont,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetFont {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "font", kind: "set-font", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_font(base, self.font.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.fonts.iter().find(|item| item.id == self.font.id) { Some(previous) => vec![PdfMutation::SetFont(SetFont { font: previous.clone() })], None => vec![PdfMutation::RemoveFont(super::remove_font::RemoveFont { id: self.font.id.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set font {}", self.font.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.font.id.clone()]
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
