//! ➖️ `remove-slide` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSlide {
    pub(crate) index: usize,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for RemoveSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "slide", kind: "remove-slide", record: "RemoveSlide" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as protocol::Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::RemoveSlide(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Vec<PptxMutation> {
        agg_inverse(&PptxMutation::RemoveSlide(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-slide".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
