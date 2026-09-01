//! ➕️️ `insert-frame` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertFrame {
    pub(crate) index: usize,
    pub(crate) frame: SemioImageFrame,
}

impl protocol::MutationKind<SemioImageSnapshot, SemioImageMutation> for InsertFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "frame", kind: "insert-frame", record: "InsertFrame" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<SemioImageMutation as protocol::Mutation<SemioImageSnapshot>>::Diff> {
        agg_diff(&SemioImageMutation::InsertFrame(self.clone()), base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
        agg_inverse(&SemioImageMutation::InsertFrame(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-frame".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
