//! 🔷️ `insert-shape` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertShape {
    pub(crate) vacancy: PptxXmlVacancyAddress,
    pub(crate) shape: XmlNode,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for InsertShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "shape", kind: "insert-shape", record: "InsertShape" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::insert_shape_plan(base, &self.vacancy, &self.shape))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::insert_shape_plan(base, &self.vacancy, &self.shape)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert shape", "Form einfügen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.vacancy.container.part_path.clone()).chain(self.vacancy.container.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.vacancy.index.to_string())).collect()
    }
}
//#endregion 🔖️Payload
