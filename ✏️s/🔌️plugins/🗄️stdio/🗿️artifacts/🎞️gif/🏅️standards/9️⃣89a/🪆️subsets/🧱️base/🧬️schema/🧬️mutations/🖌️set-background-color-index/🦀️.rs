//! 🖌️ `set-background-color-index` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-background-color-index")]
pub struct SetBackgroundColorIndex {
    pub(crate) index: u8,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetBackgroundColorIndex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "background-color-index", kind: "set-background-color-index", record: "SetBackgroundColorIndex" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::SetBackgroundColorIndex(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::SetBackgroundColorIndex(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-background-color-index", "Hintergrundfarbenindex setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
