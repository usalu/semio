//! ✍️ `set-text` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetText {
    pub(crate) path: NodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for SetText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text", kind: "set-text", record: "SetText" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgBasicMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgBasicMutation::SetText(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
        agg_inverse(&SvgBasicMutation::SetText(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-text".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
