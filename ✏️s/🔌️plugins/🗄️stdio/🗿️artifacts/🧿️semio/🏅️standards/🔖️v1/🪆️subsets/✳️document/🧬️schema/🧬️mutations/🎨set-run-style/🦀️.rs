//! 🎨 `set-run-style` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunStyle {
    pub(crate) path: DocBlockPath,
    pub(crate) run_index: usize,
    pub(crate) style: RunStyle,
}

impl protocol::MutationKind<SemioDocumentSnapshot, SemioDocumentMutation> for SetRunStyle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-style", kind: "set-run-style", record: "SetRunStyle" };

    fn diff(&self, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<<SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::Diff> {
        agg_diff(&SemioDocumentMutation::SetRunStyle(self.clone()), base)
    }
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Vec<SemioDocumentMutation> {
        agg_inverse(&SemioDocumentMutation::SetRunStyle(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-run-style".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
