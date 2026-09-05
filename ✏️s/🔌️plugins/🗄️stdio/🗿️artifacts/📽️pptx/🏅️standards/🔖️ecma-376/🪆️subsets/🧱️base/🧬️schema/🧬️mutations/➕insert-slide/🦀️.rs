//! ➕️ `insert-slide` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertSlide {
    pub(crate) index: usize,
    pub(crate) slide: PptxSlide,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for InsertSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "slide", kind: "insert-slide", record: "InsertSlide" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as protocol::Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::InsertSlide(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Vec<PptxMutation> {
        agg_inverse(&PptxMutation::InsertSlide(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-slide".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
