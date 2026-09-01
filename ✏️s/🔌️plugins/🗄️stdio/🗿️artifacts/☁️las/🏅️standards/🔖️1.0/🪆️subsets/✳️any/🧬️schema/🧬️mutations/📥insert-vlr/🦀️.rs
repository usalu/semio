//! 📥️ `insert-vlr` — its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! ➕️ Inserts a fully-specified VLR at `index` (final position, clamped to `len`).
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertVlr {
    pub index: usize,
    pub vlr: LasVlr,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for InsertVlr {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vlr", kind: "insert-vlr", record: "InsertVlr" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::Mutation<LasSnapshot>>::Diff> {
        agg_diff(&LasMutation::InsertVlr(self.clone()), base)
    }
    fn inverse(&self, base: &LasSnapshot) -> Vec<LasMutation> {
        agg_inverse(&LasMutation::InsertVlr(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-vlr".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
