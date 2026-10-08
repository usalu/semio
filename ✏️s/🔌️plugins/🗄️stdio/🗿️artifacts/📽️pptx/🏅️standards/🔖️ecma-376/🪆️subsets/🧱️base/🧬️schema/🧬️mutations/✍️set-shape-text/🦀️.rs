//! ✍️ `set-shape-text` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShapeText {
    pub(crate) address: PptxShapeAddress,
    pub(crate) text: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetShapeText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-text", kind: "set-shape-text", record: "SetShapeText" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::set_shape_text_plan(base, &self.address, &self.text))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::set_shape_text_plan(base, &self.address, &self.text)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set shape text", "Text der Form setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.node.part_path.clone()).chain(self.address.node.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
