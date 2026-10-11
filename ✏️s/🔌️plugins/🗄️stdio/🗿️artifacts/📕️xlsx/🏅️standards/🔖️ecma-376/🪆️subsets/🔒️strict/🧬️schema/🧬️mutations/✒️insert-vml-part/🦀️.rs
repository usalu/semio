//! 🏷️ `insert-vml-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVmlPart {
    pub(crate) path: String,
    pub(crate) document: XmlDocument,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) override_index: Option<usize>,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxStrictMutation> for InsertVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vml-part", kind: "insert-vml-part", record: "InsertVmlPart" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        protocol::MutationOutcome::new(diff_insert_vml_part(base, &self.path, &self.document, self.index, self.override_index))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxStrictMutation>, semio_framework_value::ValueError> {
        let path = self.path.trim_start_matches('/');
        if base.xml_part(path).is_some() || base.opc.part(path).is_some() {
            return Ok(Vec::new());
        }
        Ok(vec![XlsxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: self.path.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert VML part", "VML-Paketteil einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
