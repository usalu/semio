//! 🪒 `remove-slide` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSlide {
    pub(crate) index: usize,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for RemoveSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "slide", kind: "remove-slide", record: "RemoveSlide" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::Diff> {
        agg_diff(&SemioPresentationMutation::RemoveSlide(self.clone()), base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
        agg_inverse(&SemioPresentationMutation::RemoveSlide(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-slide".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
