//! 🗑️ Direct remove-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovePage {
    pub index: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl RemovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        base.pages.len() > 1 && self.index < base.pages.len()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "RemovedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.remove-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.index], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::InsertPage(super::InsertPage { index: self.index, page: base.pages[self.index].clone() })]
    }

    fn label(&self) -> String {
        "remove page".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/round-trips-the-concrete-inverse/🦀️component.rs"]
mod tests_round_trips_the_concrete_inverse;

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    /// 🚫️ The refusal branch the committed vector cannot express, because a refused mutation
    /// produces no after-state to commit: addressed against a document with no pages at all,
    /// `remove-page` must raise, leave the document untouched, and offer no undo.
    #[test]
    fn missing_page_refuses_without_inverse_or_state_change() {
        let mutation: PdfMutation = serde_json::from_str(include_str!("🧪️tests/round-trips-the-concrete-inverse/🦠️mutation/🔣️component.json")).expect("committed remove-page payload decodes");
        let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
        let mut state = base.clone();
        assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty(), "remove-page: an unaddressable page must be refused");
        assert_eq!(state, base, "remove-page: a refused mutation must leave the document untouched");
        assert!(mutation.inverse(&base).is_empty(), "remove-page: a refused mutation has nothing to undo");
    }
}
//#endregion 🧪️Tests
