//! ➖️ `remove-element` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for RemoveElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemoveElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgBasicMutation as Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgBasicMutation::RemoveElement(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
        agg_inverse(&SvgBasicMutation::RemoveElement(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("remove-element", "Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
