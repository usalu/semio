//! 🔨️ `remove-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElement {
    pub(crate) id: String,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for RemoveElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemoveElement" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::RemoveElement(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::RemoveElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
