//! ✍️ `set-shape-text` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetShapeText {
    pub(crate) slide_index: usize,
    pub(crate) shape_index: usize,
    pub(crate) text_frame: Vec<PptxParagraph>,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetShapeText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-text", kind: "set-shape-text", record: "SetShapeText" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as protocol::Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::SetShapeText(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Vec<PptxMutation> {
        agg_inverse(&PptxMutation::SetShapeText(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-shape-text".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
