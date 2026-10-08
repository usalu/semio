//! 🏷️ `insert-vml-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVmlPart {
    pub(crate) path: String,
    pub(crate) document: XmlDocument,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for InsertVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vml-part", kind: "insert-vml-part", record: "InsertVmlPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_insert_vml_part(base, &self.path, &self.document))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        let path = self.path.trim_start_matches('/');
        if base.xml_part(path).is_some() || base.opc.part(path).is_some() {
            return Ok(Vec::new());
        }
        Ok(vec![DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: self.path.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert VML part", "VML-Paketteil einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
