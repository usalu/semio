//! 🔀️ `move-image` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveImage {
    pub(crate) from: usize,
    pub(crate) to: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for MoveImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "image", kind: "move-image", record: "MoveImage" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as protocol::Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::MoveImage(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::MoveImage(self.clone()), base)
    }
    fn label(&self) -> String {
        "move-image".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
