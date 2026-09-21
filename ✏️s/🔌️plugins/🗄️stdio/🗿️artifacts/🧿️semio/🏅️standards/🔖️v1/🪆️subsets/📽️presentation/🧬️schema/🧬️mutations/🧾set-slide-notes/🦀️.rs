//! 🧾 `set-slide-notes` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSlideNotes {
    pub(crate) index: usize,
    pub(crate) notes: Vec<DocBlock>,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for SetSlideNotes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "slide-notes", kind: "set-slide-notes", record: "SetSlideNotes" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::Diff> {
        agg_diff(&SemioPresentationMutation::SetSlideNotes(self.clone()), base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
        agg_inverse(&SemioPresentationMutation::SetSlideNotes(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-slide-notes", "Foliennotizen setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
