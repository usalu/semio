//! 🧬️ Transparent XmlMutation aggregate.
use crate::artifacts::xml::schema::diff::XmlDiff;
use crate::artifacts::xml::XmlSnapshot;

pub use super::set_declaration::{SetDeclarationMutation, SetDeclarationPayload};
pub use super::set_doctype::{SetDoctypeMutation, SetDoctypePayload};
pub use super::insert_element::{InsertElementMutation, InsertElementPayload};
pub use super::remove_element::{RemoveElementMutation, RemoveElementPayload};
pub use super::set_attribute::{SetAttributeMutation, SetAttributePayload};
pub use super::set_text::{SetTextMutation, SetTextPayload};
pub use crate::artifacts::xml::schema::mutation_support::XmlNodePath;

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
}

pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) -> protocol::MutationOutcome<XmlDiff> {
    let outcome = <XmlMutation as protocol::Mutation<XmlSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) { *snapshot = next; }
    outcome
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<XmlMutation> {
    use crate::artifacts::xml::schema::snapshot::XmlNode;
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
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    #[test]
    fn aggregate_roster_is_exact() { assert_eq!(XmlMutation::kinds().len(), 6); }
}
