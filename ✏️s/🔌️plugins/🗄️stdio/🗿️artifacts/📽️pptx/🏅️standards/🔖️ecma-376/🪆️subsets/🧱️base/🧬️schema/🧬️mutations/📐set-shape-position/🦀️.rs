//! 📐️ `set-shape-position` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShapePosition {
    pub(crate) address: PptxShapeAddress,
    pub(crate) position: PptxTransform,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetShapePosition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-position", kind: "set-shape-position", record: "SetShapePosition" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::SetShapePosition(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&PptxMutation::SetShapePosition(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set shape position", "Position der Form setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.node.part_path.clone()).chain(self.address.node.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
