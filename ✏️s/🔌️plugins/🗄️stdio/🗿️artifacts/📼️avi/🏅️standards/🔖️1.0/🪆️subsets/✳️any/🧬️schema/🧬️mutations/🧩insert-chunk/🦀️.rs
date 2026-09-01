//! 🧩️ `insert-chunk` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertChunk {
    pub stream_index: usize,
    pub index: usize,
    pub chunk: AviChunk,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for InsertChunk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "chunk", kind: "insert-chunk", record: "InsertChunk" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::Mutation<AviSnapshot>>::Diff> {
        agg_diff(&AviMutation::InsertChunk(self.clone()), base)
    }
    fn inverse(&self, base: &AviSnapshot) -> Vec<AviMutation> {
        agg_inverse(&AviMutation::InsertChunk(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-chunk".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
