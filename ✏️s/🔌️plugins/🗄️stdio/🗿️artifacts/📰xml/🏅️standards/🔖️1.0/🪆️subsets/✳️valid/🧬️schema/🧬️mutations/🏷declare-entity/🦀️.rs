//! 🏷️ `declare-entity` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeclareEntity {
    pub(crate) index: usize,
    pub(crate) parameter: bool,
    pub(crate) name: String,
    pub(crate) value: String,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for DeclareEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "entity", kind: "declare-entity", record: "DeclareEntity" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        agg_diff(&XmlValidMutation::DeclareEntity(self.clone()), base)
    }
    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
        agg_inverse(&XmlValidMutation::DeclareEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "declare-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
