//! 🧬️ Transparent SvgMutation aggregate.
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;

pub use super::insert_element::{InsertElementMutation, InsertElementPayload};
pub use super::remove_element::{RemoveElementMutation, RemoveElementPayload};
pub use super::set_attribute::{SetAttributeMutation, SetAttributePayload};
pub use super::set_declaration::{SetDeclarationMutation, SetDeclarationPayload};
pub use super::set_doctype::{SetDoctypeMutation, SetDoctypePayload};
pub use super::set_element_name::{SetElementNameMutation, SetElementNamePayload};
pub use super::set_text::{SetTextMutation, SetTextPayload};
pub use super::set_transform::{SetTransformMutation, SetTransformPayload};
pub use super::set_view_box::{SetViewBoxMutation, SetViewBoxPayload};

use super::set_snapshot::SetSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "s.stdio.svg")]
pub enum SvgMutation {
    SetSnapshot(SetSnapshot),
    SetDeclaration(SetDeclarationMutation),
    SetDoctype(SetDoctypeMutation),
    InsertElement(InsertElementMutation),
    RemoveElement(RemoveElementMutation),
    SetElementName(SetElementNameMutation),
    SetAttribute(SetAttributeMutation),
    SetText(SetTextMutation),
    SetViewBox(SetViewBoxMutation),
    SetTransform(SetTransformMutation),
}

pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

/// ↩️ The aggregate's own `Mutation::inverse`, reachable for a caller that cannot name the trait.
pub fn inverse_svg_mutation(mutation: &SvgMutation, base: &SvgSnapshot) -> Vec<SvgMutation> {
    <SvgMutation as protocol::Mutation<SvgSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes one leaf wire payload (a `🥒️.feature` row's `params`: the leaf's `payload_value()`, no aggregate tag) into
/// the operation of semantic kind `kind` through the derive-generated `Mutation::from_payload_value`, so a caller that
/// cannot name the trait reads the committed wire instead of re-declaring it field by field.
pub fn decode_svg_mutation_payload_json(kind: &str, payload: &str) -> Result<SvgMutation, String> {
    let value = pack::parse_json(payload).map_err(|error| error.to_string())?;
    <SvgMutation as protocol::Mutation<SvgSnapshot>>::from_payload_value(kind, pack::json_to_dsl_value(&value)).map_err(|error| error.to_string())
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SvgMutation> {
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    vec![
        SvgMutation::SetDeclaration(SetDeclarationMutation::Apply(SetDeclarationPayload { declaration: None })),
        SvgMutation::SetDoctype(SetDoctypeMutation::Apply(SetDoctypePayload { doctype: None })),
        SvgMutation::InsertElement(InsertElementMutation::Apply(InsertElementPayload { parent: Vec::new(), index: 0, node: XmlNode::Text { text: "inserted".into() } })),
        SvgMutation::RemoveElement(RemoveElementMutation::Apply(RemoveElementPayload { parent: Vec::new(), index: 0 })),
        SvgMutation::SetElementName(SetElementNameMutation::Apply(SetElementNamePayload { path: Vec::new(), name: "svg".into() })),
        SvgMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: Vec::new(), name: "attribute".into(), value: Some("value".into()) })),
        SvgMutation::SetText(SetTextMutation::Apply(SetTextPayload { path: Vec::new(), text: "text".into() })),
        SvgMutation::SetViewBox(SetViewBoxMutation::Apply(SetViewBoxPayload { path: Vec::new(), view_box: None })),
        SvgMutation::SetTransform(SetTransformMutation::Apply(SetTransformPayload { path: Vec::new(), transform: None })),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
