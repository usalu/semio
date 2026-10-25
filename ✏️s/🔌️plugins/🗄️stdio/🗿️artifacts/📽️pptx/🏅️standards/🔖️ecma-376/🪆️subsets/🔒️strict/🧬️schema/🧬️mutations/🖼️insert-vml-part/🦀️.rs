//! 📐️ `insert-vml-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVmlPart {
    pub path: String,
    pub document: XmlDocument,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub override_index: Option<usize>,
}

impl protocol::MutationKind<PptxSnapshot, PptxStrictMutation> for InsertVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vml-part", kind: "insert-vml-part", record: "InsertVmlPart" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert VML part", "VML-Paketteil einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<PptxSnapshot, PptxStrictMutation> for InsertVmlPart {
    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        protocol::MutationOutcome::new(diff_insert_vml_part(base, &self.path, &self.document, self.index, self.override_index))
    }
    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxStrictMutation>, semio_framework_value::ValueError> {
        Ok(insert_vml_part_inverse(base, &self.path))
    }
}
//#endregion 🔖️Payload
