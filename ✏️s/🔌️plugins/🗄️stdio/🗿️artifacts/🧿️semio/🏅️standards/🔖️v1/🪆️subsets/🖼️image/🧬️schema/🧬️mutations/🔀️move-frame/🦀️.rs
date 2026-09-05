//! 🔀️️ `move-frame` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveFrame {
    pub(crate) from: usize,
    pub(crate) to: usize,
}

impl protocol::MutationKind<SemioImageSnapshot, SemioImageMutation> for MoveFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "frame", kind: "move-frame", record: "MoveFrame" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<SemioImageMutation as protocol::Mutation<SemioImageSnapshot>>::Diff> {
        agg_diff(&SemioImageMutation::MoveFrame(self.clone()), base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
        agg_inverse(&SemioImageMutation::MoveFrame(self.clone()), base)
    }
    fn label(&self) -> String {
        "move-frame".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
