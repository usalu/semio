//! ⚡️ Sourcing curate artifact — the operation type + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::curate::diff::SourcingDiff;
use crate::artifacts::curate::CurateDocument;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🛒️ Curate document operation: currently always a wholesale swap — every action recomputes the
/// full document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SourcingMutation {
    SetDocument {
        #[dsl(block)]
        document: CurateDocument,
    },
}





impl Mutation<CurateDocument> for SourcingMutation {
    type Diff = SourcingDiff;

    fn diff(&self, _projection: &CurateDocument) -> Self::Diff {
        match self {
            SourcingMutation::SetDocument { document } => SourcingDiff { document: Some(document.clone()) },
        }
    }

    fn inverse(&self, projection: &CurateDocument) -> Vec<Self> {
        match self {
            SourcingMutation::SetDocument { .. } => vec![SourcingMutation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    fn sample_document() -> CurateDocument {
        CurateDocument { stock: crate::artifacts::curate::engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingMutation::SetDocument { document: sample_document() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingMutation::SetDocument { document: CurateDocument::default() });
    }

    /// ⚖️ LAW: `backwards()` restores the pre-operation projection.
    #[test]
    fn set_document_backwards_restores_the_base_projection() {
        let base = sample_document();
        let operation = SourcingMutation::SetDocument { document: CurateDocument::default() };
        let forward = operation.diff(&base).apply(&base);
        assert_eq!(forward, CurateDocument::default());
        let restored = operation.inverse(&base).iter().fold(forward, |projection, inverse| inverse.diff(&projection).apply(&projection));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests


pub fn apply_sourcing_mutation(projection: &mut CurateDocument, mutation: &SourcingMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_sourcing_mutation(projection: &CurateDocument, mutation: &SourcingMutation) -> Vec<SourcingMutation> {
    mutation.inverse(projection)
}
