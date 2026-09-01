//! 📦️ `set-part` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPart {
    pub(crate) path: String,
    pub(crate) content_type: String,
    pub(crate) bytes: Vec<u8>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "part", kind: "set-part", record: "SetPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::SetPart(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::SetPart(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-part".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
