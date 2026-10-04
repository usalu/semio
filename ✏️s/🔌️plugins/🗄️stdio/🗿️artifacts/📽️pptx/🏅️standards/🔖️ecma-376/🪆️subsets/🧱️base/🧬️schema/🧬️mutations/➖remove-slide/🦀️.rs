//! ➖️ `remove-slide` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSlide {
    pub(crate) address: PptxSlideAddress,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for RemoveSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "slide", kind: "remove-slide", record: "RemoveSlide" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::RemoveSlide(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&PptxMutation::RemoveSlide(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove slide", "Folie entfernen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.entry.part_path.clone()).chain(self.address.entry.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
