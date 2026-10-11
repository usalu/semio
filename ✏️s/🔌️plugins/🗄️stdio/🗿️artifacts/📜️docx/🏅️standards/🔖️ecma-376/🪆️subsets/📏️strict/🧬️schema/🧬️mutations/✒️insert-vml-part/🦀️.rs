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

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for InsertVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vml-part", kind: "insert-vml-part", record: "InsertVmlPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_insert_vml_part(base, &self.path, &self.document, self.index, self.override_index))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(insert_vml_part_inverse(base, &self.path))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert VML part", "VML-Paketteil einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
