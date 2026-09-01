//! 📜️ `declare-doctype` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeclareDoctype {
    pub(crate) external_id: Option<XmlExternalId>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for DeclareDoctype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "doctype", kind: "declare-doctype", record: "DeclareDoctype" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        agg_diff(&XmlValidMutation::DeclareDoctype(self.clone()), base)
    }
    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
        agg_inverse(&XmlValidMutation::DeclareDoctype(self.clone()), base)
    }
    fn label(&self) -> String {
        "declare-doctype".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
