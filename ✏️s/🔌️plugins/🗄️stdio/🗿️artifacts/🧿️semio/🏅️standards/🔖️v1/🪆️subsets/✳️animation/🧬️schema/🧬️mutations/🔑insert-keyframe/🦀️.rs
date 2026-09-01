//! 🔑️ `insert-keyframe` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertKeyframe {
    pub(crate) timeline_index: usize,
    pub(crate) channel_index: usize,
    pub(crate) index: usize,
    pub(crate) keyframe: AnimKeyframe,
}

impl protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation> for InsertKeyframe {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "keyframe", kind: "insert-keyframe", record: "InsertKeyframe" };

    fn diff(&self, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<<SemioAnimationMutation as protocol::Mutation<SemioAnimationSnapshot>>::Diff> {
        agg_diff(&SemioAnimationMutation::InsertKeyframe(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
        agg_inverse(&SemioAnimationMutation::InsertKeyframe(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-keyframe".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
