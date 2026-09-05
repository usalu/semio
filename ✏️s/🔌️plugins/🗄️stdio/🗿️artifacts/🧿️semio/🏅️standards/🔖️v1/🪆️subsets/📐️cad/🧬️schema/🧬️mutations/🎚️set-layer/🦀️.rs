//! 🎚️ `set-layer` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLayer {
    pub(crate) name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) color_index: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) line_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) visible: Option<bool>,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for SetLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer", record: "SetLayer" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::Diff> {
        agg_diff(&SemioCadMutation::SetLayer(self.clone()), base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
        agg_inverse(&SemioCadMutation::SetLayer(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-layer".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
