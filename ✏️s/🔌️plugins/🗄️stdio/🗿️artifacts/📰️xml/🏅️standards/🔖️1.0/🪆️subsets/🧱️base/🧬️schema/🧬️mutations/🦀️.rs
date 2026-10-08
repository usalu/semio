//! 🧬️ Transparent XmlMutation aggregate.
use crate::schema::diff::XmlDiff;
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;

pub use super::insert_element::{InsertElementMutation, InsertElementPayload};
pub use super::remove_element::{RemoveElementMutation, RemoveElementPayload};
pub use super::set_attribute::{SetAttributeMutation, SetAttributePayload};
pub use super::set_declaration::{SetDeclarationMutation, SetDeclarationPayload};
pub use super::set_doctype::{SetDoctypeMutation, SetDoctypePayload};
pub use super::set_text::{SetTextMutation, SetTextPayload};
pub use crate::schema::mutation_support::XmlNodePath;


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




#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<XmlMutation> {
    use crate::schema::snapshot::XmlNode;
    vec![
        XmlMutation::SetDeclaration(SetDeclarationPayload { declaration: None }),
        XmlMutation::SetDoctype(SetDoctypePayload { doctype: None }),
        XmlMutation::InsertElement(InsertElementPayload { path: XmlNodePath::root(), index: 0, node: XmlNode::Text { text: "inserted".into() } }),
        XmlMutation::RemoveElement(RemoveElementPayload { path: XmlNodePath::root(), index: 0 }),
        XmlMutation::SetAttribute(SetAttributePayload { path: XmlNodePath::root(), name: "attribute".into(), value: Some("value".into()), index: None }),
        XmlMutation::SetText(SetTextPayload { path: XmlNodePath::root(), text: "text".into() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
