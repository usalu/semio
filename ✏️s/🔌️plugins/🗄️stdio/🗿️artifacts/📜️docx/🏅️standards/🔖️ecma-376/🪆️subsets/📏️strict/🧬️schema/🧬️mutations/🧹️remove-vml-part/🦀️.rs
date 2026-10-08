//! 📐️ `remove-vml-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveVmlPart {
    pub(crate) path: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for RemoveVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "vml-part", kind: "remove-vml-part", record: "RemoveVmlPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_remove_vml_part(base, &self.path))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(match base.xml_part(self.path.trim_start_matches('/')) {
            Some(part) => vec![DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: self.path.clone(), document: part.materialize_document_exact()? })],
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
