//! 🔶️ `remove-shape` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveShape {
    pub(crate) address: PptxShapeAddress,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for RemoveShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "shape", kind: "remove-shape", record: "RemoveShape" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::remove_shape_plan(base, &self.address))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::remove_shape_plan(base, &self.address)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove shape", "Form entfernen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.node.part_path.clone()).chain(self.address.node.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
