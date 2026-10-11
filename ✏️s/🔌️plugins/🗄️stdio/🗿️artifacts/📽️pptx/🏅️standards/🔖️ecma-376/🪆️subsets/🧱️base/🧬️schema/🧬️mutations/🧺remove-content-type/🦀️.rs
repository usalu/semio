//! 🧺 `remove-content-type` — removes one `[Content_Types].xml` entry (an extension default or a part-name override). It builds its own sparse diff from its payload and
//! reads of `base`; the inverse writes the exact content type back at the position it held.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveContentType {
    #[value(default)]
    pub(crate) is_override: bool,
    pub(crate) name: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for RemoveContentType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "content-type", kind: "remove-content-type", record: "RemoveContentType" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        opc_layer::outcome(opc_layer::with_package(base, |opc| opc_layer::content_type_removal_diff(opc, self.is_override, &self.name)))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        let previous = opc_layer::with_package(base, |opc| opc_layer::content_type_at(opc, self.is_override, &self.name)).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(previous.map(|(at, content_type)| PptxMutation::SetContentType(set_content_type::SetContentType { is_override: self.is_override, name: self.name.clone(), content_type, index: Some(at) })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove content type", "Inhaltstyp entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["opc".to_string(), "contentTypes".to_string(), if self.is_override { "overrides" } else { "defaults" }.to_string(), self.name.clone()]
    }
}
//#endregion 🔖️Payload

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
