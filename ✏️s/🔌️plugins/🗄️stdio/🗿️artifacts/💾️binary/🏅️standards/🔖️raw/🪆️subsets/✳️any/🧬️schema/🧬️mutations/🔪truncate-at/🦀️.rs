//! 🔪️ `truncate-at` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct TruncateAt {
    pub offset: usize,
}

impl protocol::MutationKind<BinarySnapshot, BinaryMutation> for TruncateAt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "trailing-bytes", kind: "truncate-at", record: "TruncateAt" };

    fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<<BinaryMutation as protocol::Mutation<BinarySnapshot>>::Diff> {
        agg_diff(&BinaryMutation::TruncateAt(self.clone()), base)
    }
    fn inverse(&self, base: &BinarySnapshot) -> Vec<BinaryMutation> {
        agg_inverse(&BinaryMutation::TruncateAt(self.clone()), base)
    }
    fn label(&self) -> String {
        "truncate-at".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
