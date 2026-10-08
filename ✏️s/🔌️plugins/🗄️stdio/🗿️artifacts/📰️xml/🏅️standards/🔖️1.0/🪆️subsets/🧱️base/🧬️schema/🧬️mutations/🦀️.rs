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

pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) -> protocol::MutationOutcome<XmlDiff> {
    let outcome = <XmlMutation as protocol::Mutation<XmlSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to `next`: the declaration and doctype that moved, then the root tree walked in place — an element
/// of the same name keeps its identity (attributes set or removed by name, children paired by index), a text node is re-set, any
/// other change replaces the node as remove-then-insert, and the diverging child tails are removed or inserted. The prolog, the epilog
/// and the root itself have no leaf; the exact net refuses an edit that needs one.
pub fn net_mutations(base: &XmlSnapshot, next: &XmlSnapshot) -> Vec<XmlMutation> {
    let mut leaves = Vec::new();
    if base.doc.declaration != next.doc.declaration {
        leaves.push(XmlMutation::SetDeclaration(SetDeclarationPayload { declaration: next.doc.declaration.clone() }));
    }
    if base.doc.doctype != next.doc.doctype {
        leaves.push(XmlMutation::SetDoctype(SetDoctypePayload { doctype: next.doc.doctype.clone() }));
    }
    if let (Some(before), Some(after)) = (&base.doc.root, &next.doc.root) {
        net_node(&mut Vec::new(), before, after, &mut leaves);
    }
    leaves
}

fn net_node(path: &mut Vec<usize>, before: &XmlNode, after: &XmlNode, leaves: &mut Vec<XmlMutation>) {
    match (before, after) {
        (XmlNode::Text { text: old }, XmlNode::Text { text }) if old != text => leaves.push(XmlMutation::SetText(SetTextPayload { path: XmlNodePath(path.clone()), text: text.clone() })),
        (XmlNode::Element { name: old_name, attrs: old_attrs, children: old_children }, XmlNode::Element { name, attrs, children }) if old_name == name => {
            for attribute in old_attrs.iter().filter(|attribute| !attrs.iter().any(|kept| kept.name == attribute.name)) {
                leaves.push(XmlMutation::SetAttribute(SetAttributePayload { path: XmlNodePath(path.clone()), name: attribute.name.clone(), value: None, index: None }));
            }
            for (index, attribute) in attrs.iter().enumerate().filter(|(_, attribute)| old_attrs.iter().find(|old| old.name == attribute.name).is_none_or(|old| old.value != attribute.value)) {
                leaves.push(XmlMutation::SetAttribute(SetAttributePayload { path: XmlNodePath(path.clone()), name: attribute.name.clone(), value: Some(attribute.value.clone()), index: Some(index) }));
            }
            let paired = old_children.len().min(children.len());
            for (index, (old, new)) in old_children.iter().zip(children).enumerate().filter(|(_, (old, new))| old != new) {
                let same_shape = matches!((old, new), (XmlNode::Text { .. }, XmlNode::Text { .. })) || matches!((old, new), (XmlNode::Element { name: left, .. }, XmlNode::Element { name: right, .. }) if left == right);
                if same_shape {
                    path.push(index);
                    net_node(path, old, new, leaves);
                    path.pop();
                } else {
                    leaves.push(XmlMutation::RemoveElement(RemoveElementPayload { path: XmlNodePath(path.clone()), index }));
                    leaves.push(XmlMutation::InsertElement(InsertElementPayload { path: XmlNodePath(path.clone()), index, node: new.clone() }));
                }
            }
            leaves.extend((paired..old_children.len()).rev().map(|index| XmlMutation::RemoveElement(RemoveElementPayload { path: XmlNodePath(path.clone()), index })));
            leaves.extend(children.iter().enumerate().skip(paired).map(|(index, node)| XmlMutation::InsertElement(InsertElementPayload { path: XmlNodePath(path.clone()), index, node: node.clone() })));
        }
        _ => {}
    }
}
//#endregion 🔖️Net

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
