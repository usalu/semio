//! 📦️ `set-part` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

/// 📦️ Authoritative package content, independent of physical XML and binary encodings.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocxPartContent {
    Xml { document: semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument },
    Binary { bytes: Vec<u8> },
}

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPart {
    pub(crate) path: String,
    pub(crate) content_type: String,
    pub(crate) payload: DocxPartContent,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) override_index: Option<usize>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "part", kind: "set-part", record: "SetPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        part_outcome(set_part_diff(base, &self.path, &self.content_type, &self.payload, self.index, self.override_index))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
        set_part_inverse(base, &self.path)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set part", "Paketteil setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
