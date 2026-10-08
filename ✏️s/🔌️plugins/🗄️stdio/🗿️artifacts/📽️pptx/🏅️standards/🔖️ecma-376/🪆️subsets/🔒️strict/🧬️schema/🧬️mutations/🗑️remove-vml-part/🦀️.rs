//! 📏️ `remove-vml-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveVmlPart {
    pub(crate) path: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxStrictMutation> for RemoveVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "vml-part", kind: "remove-vml-part", record: "RemoveVmlPart" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        protocol::MutationOutcome::new(diff_remove_vml_part(base, &self.path))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxStrictMutation>, semio_framework_value::ValueError> {
        let path = self.path.trim_start_matches('/');
        Ok(match base.xml_parts.iter().position(|part| part.path == path) {
            Some(index) => vec![PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: self.path.clone(), document: base.xml_parts[index].document.clone(), index: Some(index) })],
            None => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove VML part", "VML-Paketteil entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
