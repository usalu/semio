//! 🧬️ Transparent XmlMutation aggregate.
use crate::schema::diff::XmlDiff;
use crate::XmlSnapshot;

pub use super::insert_element::{InsertElementMutation, InsertElementPayload};
pub use super::remove_element::{RemoveElementMutation, RemoveElementPayload};
pub use super::set_attribute::{SetAttributeMutation, SetAttributePayload};
pub use super::set_declaration::{SetDeclarationMutation, SetDeclarationPayload};
pub use super::set_doctype::{SetDoctypeMutation, SetDoctypePayload};
pub use super::set_text::{SetTextMutation, SetTextPayload};
pub use crate::schema::mutation_support::XmlNodePath;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = XmlSnapshot, diff = XmlDiff, schema = "s.stdio.xml")]
pub enum XmlMutation {
    SetDeclaration(SetDeclarationMutation),
    SetDoctype(SetDoctypeMutation),
    InsertElement(InsertElementMutation),
    RemoveElement(RemoveElementMutation),
    SetAttribute(SetAttributeMutation),
    SetText(SetTextMutation),
    SetSnapshot(set_snapshot::SetSnapshot),
}

pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) -> protocol::MutationOutcome<XmlDiff> {
    let outcome = <XmlMutation as protocol::Mutation<XmlSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

/// ↩️ The aggregate's own `Mutation::inverse`, reachable for a caller that cannot name the trait.
pub fn inverse_xml_mutation(mutation: &XmlMutation, base: &XmlSnapshot) -> Vec<XmlMutation> {
    <XmlMutation as protocol::Mutation<XmlSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes one leaf wire payload (a `🥒️.feature` row's `params`: the leaf's `payload_value()`, no aggregate tag) into
/// the operation of semantic kind `kind` through the derive-generated `Mutation::from_payload_value`, so a caller that
/// cannot name the trait reads the committed wire instead of re-declaring it field by field.
pub fn decode_xml_mutation_payload_json(kind: &str, payload: &str) -> Result<XmlMutation, String> {
    let value = pack::parse_json(payload).map_err(|error| error.to_string())?;
    <XmlMutation as protocol::Mutation<XmlSnapshot>>::from_payload_value(kind, pack::json_to_dsl_value(&value)).map_err(|error| error.to_string())
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<XmlMutation> {
    use crate::schema::snapshot::XmlNode;
    vec![
        XmlMutation::SetDeclaration(SetDeclarationMutation::Apply(SetDeclarationPayload { declaration: None })),
        XmlMutation::SetDoctype(SetDoctypeMutation::Apply(SetDoctypePayload { doctype: None })),
        XmlMutation::InsertElement(InsertElementMutation::Apply(InsertElementPayload { path: XmlNodePath::root(), index: 0, node: XmlNode::Text { text: "inserted".into() } })),
        XmlMutation::RemoveElement(RemoveElementMutation::Apply(RemoveElementPayload { path: XmlNodePath::root(), index: 0 })),
        XmlMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: XmlNodePath::root(), name: "attribute".into(), value: Some("value".into()) })),
        XmlMutation::SetText(SetTextMutation::Apply(SetTextPayload { path: XmlNodePath::root(), text: "text".into() })),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
