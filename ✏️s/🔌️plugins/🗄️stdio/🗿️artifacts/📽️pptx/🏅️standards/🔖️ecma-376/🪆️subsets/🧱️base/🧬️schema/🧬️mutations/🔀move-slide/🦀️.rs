//! 🔀️ `move-slide` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveSlide {
    pub(crate) address: PptxSlideAddress,
    pub(crate) destination_index: usize,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for MoveSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "slide", kind: "move-slide", record: "MoveSlide" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::move_slide_plan(base, &self.address, self.destination_index))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::move_slide_plan(base, &self.address, self.destination_index)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move slide", "Folie verschieben")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.entry.part_path.clone()).chain(self.address.entry.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
