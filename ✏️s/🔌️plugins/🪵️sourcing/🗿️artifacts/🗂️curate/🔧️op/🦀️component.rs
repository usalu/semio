//! ⚡️ Sourcing curate artifact — the operation type + laws (constitutional: op).

use crate::artifacts::curate::diff::SourcingDiff;
use crate::artifacts::curate::CurateDocument;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🛒️ Curate document operation: currently always a wholesale swap — every action recomputes the
/// full document and this carries it, with a true inverse restoring the exact prior document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SourcingOperation {
    SetDocument {
        #[dsl(block)]
        document: CurateDocument,
    },
}

impl Operation<CurateDocument> for SourcingOperation {
    type Diff = SourcingDiff;

    fn diff(&self, _projection: &CurateDocument) -> Self::Diff {
        match self {
            SourcingOperation::SetDocument { document } => SourcingDiff { document: Some(document.clone()) },
        }
    }

    fn backwards(&self, projection: &CurateDocument) -> Vec<Self> {
        match self {
            SourcingOperation::SetDocument { .. } => vec![SourcingOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OperationDiff;

    fn sample_document() -> CurateDocument {
        CurateDocument { stock: crate::artifacts::curate::engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: sample_document() });
        store::test_support::assert_op_text_binary_equivalence(&SourcingOperation::SetDocument { document: CurateDocument::default() });
    }

    /// ⚖️ LAW: `backwards()` restores the pre-operation projection.
    #[test]
    fn set_document_backwards_restores_the_base_projection() {
        let base = sample_document();
        let operation = SourcingOperation::SetDocument { document: CurateDocument::default() };
        let forward = operation.diff(&base).apply(&base);
        assert_eq!(forward, CurateDocument::default());
        let restored = operation.backwards(&base).iter().fold(forward, |projection, inverse| inverse.diff(&projection).apply(&projection));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
