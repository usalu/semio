//! 🔀️ Authoritative PDF mutation payload, diff, inverse, and tests for `move-page`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MovePage {
    pub from: usize,
    pub to: usize,
}

impl MutationKind<PdfSnapshot, PdfMutation> for MovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "page", kind: "move-page", record: "Move" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_move_page(base, self.from, self.to))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if base.pages.get(self.from).is_none() { Vec::new() } else { vec![PdfMutation::MovePage(MovePage { from: self.to.min(base.pages.len().saturating_sub(1)), to: self.from })] }
    }

    fn label(&self) -> String {
        format!("Move page {} to {}", self.from, self.to)
    }

    fn target(&self) -> Vec<String> {
        vec![self.from.to_string(), self.to.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<MovePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "move-page");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
