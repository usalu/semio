//! 📚️ `set-internal-subset` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInternalSubset {
    pub(crate) declarations: Vec<XmlDtdDeclaration>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetInternalSubset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "internal-subset", kind: "set-internal-subset", record: "SetInternalSubset" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        agg_diff(&XmlValidMutation::SetInternalSubset(self.clone()), base)
    }
    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
        agg_inverse(&XmlValidMutation::SetInternalSubset(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-internal-subset".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
