//! 🏳️ `set-standalone` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStandalone {
    pub(crate) standalone: Option<bool>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetStandalone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "standalone", kind: "set-standalone", record: "SetStandalone" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as protocol::Mutation<XmlSnapshot>>::Diff> {
        agg_diff(&XmlValidMutation::SetStandalone(self.clone()), base)
    }
    fn inverse(&self, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
        agg_inverse(&XmlValidMutation::SetStandalone(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-standalone".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
