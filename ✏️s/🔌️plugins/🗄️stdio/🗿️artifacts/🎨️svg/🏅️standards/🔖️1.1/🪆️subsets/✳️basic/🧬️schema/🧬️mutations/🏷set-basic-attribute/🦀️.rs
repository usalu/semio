//! 🏷️ `set-basic-attribute` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBasicAttribute {
    pub(crate) path: NodePath,
    pub(crate) name: String,
    pub(crate) value: Option<String>,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for SetBasicAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "basic-attribute", kind: "set-basic-attribute", record: "SetBasicAttribute" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgBasicMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgBasicMutation::SetBasicAttribute(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
        agg_inverse(&SvgBasicMutation::SetBasicAttribute(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-basic-attribute".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
