//! 🔪️ `truncate-at` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when this field lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "truncate-at")]
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
