//! 🔗 `set-style-based-on` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStyleBasedOn {
    pub(crate) id: String,
    pub(crate) based_on: Option<String>,
}

impl protocol::MutationKind<SemioDocumentSnapshot, SemioDocumentMutation> for SetStyleBasedOn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "style-based-on", kind: "set-style-based-on", record: "SetStyleBasedOn" };

    fn diff(&self, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<<SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::Diff> {
        agg_diff(&SemioDocumentMutation::SetStyleBasedOn(self.clone()), base)
    }
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Vec<SemioDocumentMutation> {
        agg_inverse(&SemioDocumentMutation::SetStyleBasedOn(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-style-based-on".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
