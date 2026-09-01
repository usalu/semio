//! 🌳️ `rename-document-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameDocumentElement {
    pub(crate) name: String,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for RenameDocumentElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "document-element", kind: "rename-document-element", record: "RenameDocumentElement" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        agg_diff(&XmlValidMutation::RenameDocumentElement(self.clone()), base)
    }
    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
        agg_inverse(&XmlValidMutation::RenameDocumentElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "rename-document-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
