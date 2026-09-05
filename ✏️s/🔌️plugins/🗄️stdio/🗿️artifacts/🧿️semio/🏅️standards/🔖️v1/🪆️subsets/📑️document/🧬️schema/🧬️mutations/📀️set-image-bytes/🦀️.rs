//! 🧮 `set-image-bytes` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetImageBytes {
    pub(crate) id: String,
    pub(crate) mime: String,
    pub(crate) bytes: Vec<u8>,
}

impl protocol::MutationKind<SemioDocumentSnapshot, SemioDocumentMutation> for SetImageBytes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "image-bytes", kind: "set-image-bytes", record: "SetImageBytes" };

    fn diff(&self, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<<SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::Diff> {
        agg_diff(&SemioDocumentMutation::SetImageBytes(self.clone()), base)
    }
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Vec<SemioDocumentMutation> {
        agg_inverse(&SemioDocumentMutation::SetImageBytes(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-image-bytes".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
