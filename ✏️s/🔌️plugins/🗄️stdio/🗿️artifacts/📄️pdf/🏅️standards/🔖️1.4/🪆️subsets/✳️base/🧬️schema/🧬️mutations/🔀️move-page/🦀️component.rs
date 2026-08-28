//! 🔀️ Direct move-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{
    diff::{PdfDiff, PdfPageAdded, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MovePage {
    pub from: usize,
    pub to: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl MovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        self.from < base.pages.len() && self.to < base.pages.len()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for MovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "page", kind: "move-page", record: "MovedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.move-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        if self.from == self.to {
            return MutationOutcome::new(PdfDiff::default());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.from], added: vec![PdfPageAdded { index: self.to, page: base.pages[self.from].clone() }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) || self.from == self.to {
            return Vec::new();
        }
        vec![PdfMutation::MovePage(super::MovePage { from: self.to, to: self.from })]
    }

    fn label(&self) -> String {
        "move page".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
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
    /// `move-page` must raise, leave the document untouched, and offer no undo.
    #[test]
    fn missing_page_refuses_without_inverse_or_state_change() {
        let mutation: PdfMutation = serde_json::from_str(include_str!("🧪️tests/round-trips-the-concrete-inverse/🦠️mutation/🔣️component.json")).expect("committed move-page payload decodes");
        let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
        let mut state = base.clone();
        assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty(), "move-page: an unaddressable page must be refused");
        assert_eq!(state, base, "move-page: a refused mutation must leave the document untouched");
        assert!(mutation.inverse(&base).is_empty(), "move-page: a refused mutation has nothing to undo");
    }
}
//#endregion 🧪️Tests
