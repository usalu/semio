//! 🔄️ `set-transform` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTransform {
    pub(crate) path: NodePath,
    pub(crate) transform: Option<Vec<TransformOp>>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for SetTransform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "transform", kind: "set-transform", record: "SetTransform" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgTinyMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgTinyMutation::SetTransform(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgTinyMutation> {
        agg_inverse(&SvgTinyMutation::SetTransform(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-transform".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
