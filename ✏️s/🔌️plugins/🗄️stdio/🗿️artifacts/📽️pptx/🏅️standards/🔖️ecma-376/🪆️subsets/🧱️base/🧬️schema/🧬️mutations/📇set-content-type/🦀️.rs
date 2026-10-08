//! 📇 `set-content-type` — writes one `[Content_Types].xml` entry: the extension default (`isOverride` false) or the part-name override (`isOverride` true) `name` takes
//! `contentType`; a new entry is inserted at `index` (last by default) and an existing one changed in place. It builds its own sparse diff from its payload and reads
//! of `base`; the inverse restores the previous content type at its position, or removes the entry it created.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetContentType {
    #[value(default)]
    pub(crate) is_override: bool,
    pub(crate) name: String,
    pub(crate) content_type: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetContentType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "content-type", kind: "set-content-type", record: "SetContentType" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        opc_layer::outcome(opc_layer::with_package(base, |opc| opc_layer::content_type_write_diff(opc, self.is_override, &self.name, &self.content_type, self.index)))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        let previous = opc_layer::with_package(base, |opc| opc_layer::content_type_at(opc, self.is_override, &self.name)).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(match previous {
            Some((at, content_type)) => vec![PptxMutation::SetContentType(Self { is_override: self.is_override, name: self.name.clone(), content_type, index: Some(at) })],
            None => vec![PptxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: self.is_override, name: self.name.clone() })],
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set content type", "Inhaltstyp setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["opc".to_string(), "contentTypes".to_string(), if self.is_override { "overrides" } else { "defaults" }.to_string(), self.name.clone()]
    }
}
//#endregion 🔖️Payload

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
