//! ✍️ `set-shape-text` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::xml_address::PptxShapeAddress;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShapeText {
    pub address: PptxShapeAddress,
    pub text: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetShapeText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-text", kind: "set-shape-text", record: "SetShapeText" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set shape text", "Text der Form setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.node.part_path.clone()).chain(self.address.node.node_path.iter().map(usize::to_string)).collect()
    }
}

impl protocol::ColdMutationKind<PptxSnapshot, PptxMutation> for SetShapeText {
    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::set_shape_text_plan(base, &self.address, &self.text))
    }
    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::set_shape_text_plan(base, &self.address, &self.text)))
    }
}
//#endregion 🔖️Payload
