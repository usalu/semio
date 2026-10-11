//! 🏷️ `set-worksheet-content-type` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWorksheetContentType {
    pub(crate) path: String,
    pub(crate) content_type: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) override_index: Option<usize>,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxTransitionalMutation> for SetWorksheetContentType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "worksheet-content-type", kind: "set-worksheet-content-type", record: "SetWorksheetContentType" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        protocol::MutationOutcome::new(diff_set_content_type(base, &self.path, &self.content_type, self.override_index))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxTransitionalMutation>, semio_framework_value::ValueError> {
        Ok(if base.xml_part(&self.path).is_some() { worksheet_content_type_inverse(base, &self.path) } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set worksheet content type", "Inhaltstyp des Arbeitsblatts setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
