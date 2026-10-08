//! ➕️ `insert-slide` -- builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertSlide {
    pub(crate) vacancy: PptxXmlVacancyAddress,
    pub(crate) entry: XmlNode,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for InsertSlide {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "slide", kind: "insert-slide", record: "InsertSlide" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        plan_outcome(xml_address::insert_slide_plan(base, &self.vacancy, &self.entry))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        Ok(plan_inverse(xml_address::insert_slide_plan(base, &self.vacancy, &self.entry)))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert slide", "Folie einfügen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.vacancy.container.part_path.clone()).chain(self.vacancy.container.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.vacancy.index.to_string())).collect()
    }
}
//#endregion 🔖️Payload
