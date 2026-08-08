//! 🔺️ Sourcing curate artifact — the operation diff (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::curate::CurateDocument;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️SourcingDiff
/// 🛒️ Curate document diff: currently always a wholesale swap — every action recomputes the full
/// document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcingDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<CurateDocument>,
}

impl MutationDiff<CurateDocument> for SourcingDiff {
    fn apply(&self, projection: &CurateDocument) -> CurateDocument {
        self.document.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}
//#endregion 🔖️SourcingDiff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curate::op::SourcingMutation;
    use protocol::Mutation;

    #[test]
    fn set_document_diff_carries_the_new_document_and_applies_it_verbatim() {
        let base = CurateDocument::default();
        let next = CurateDocument { stock: vec![], curated: vec![] };
        let operation = SourcingMutation::SetDocument { document: next.clone() };
        let diff: SourcingDiff = operation.diff(&base);
        assert_eq!(diff.document, Some(next.clone()));
        assert_eq!(diff.apply(&base), next);
    }

    #[test]
    fn absorb_keeps_the_later_document_when_both_diffs_carry_one() {
        let mut first = SourcingDiff { document: Some(CurateDocument { stock: vec![], curated: vec![] }) };
        let second = SourcingDiff { document: Some(CurateDocument::default()) };
        first.absorb(second.clone());
        assert_eq!(first, second);
    }
}
//#endregion 🧪️Tests
