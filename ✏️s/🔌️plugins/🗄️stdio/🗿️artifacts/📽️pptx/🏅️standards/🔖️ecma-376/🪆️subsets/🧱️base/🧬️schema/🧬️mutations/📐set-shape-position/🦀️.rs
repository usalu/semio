//! 📐️ `set-shape-position` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShapePosition {
    pub(crate) address: PptxShapeAddress,
    pub(crate) position: PptxTransform,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetShapePosition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape-position", kind: "set-shape-position", record: "SetShapePosition" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::set_shape_position_plan(base, &self.address, self.position))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::set_shape_position_plan(base, &self.address, self.position)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set shape position", "Position der Form setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.node.part_path.clone()).chain(self.address.node.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
