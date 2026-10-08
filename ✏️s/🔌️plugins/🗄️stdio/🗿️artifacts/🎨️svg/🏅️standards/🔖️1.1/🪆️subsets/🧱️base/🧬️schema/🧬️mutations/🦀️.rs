//! 🧬️ Transparent SvgMutation aggregate.
use crate::schema::diff::SvgDiff;
use crate::schema::snapshot::{SvgAttr, SvgNode};
use crate::SvgSnapshot;

pub use super::insert_element::InsertElementPayload;
pub use super::remove_element::RemoveElementPayload;
pub use super::set_attribute::SetAttributePayload;
pub use super::set_declaration::SetDeclarationPayload;
pub use super::set_doctype::SetDoctypePayload;
pub use super::set_element_name::SetElementNamePayload;
pub use super::set_text::SetTextPayload;
pub use super::set_transform::SetTransformPayload;
pub use super::set_view_box::SetViewBoxPayload;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "s.stdio.svg")]
pub enum SvgMutation {
    SetDeclaration(SetDeclarationPayload),
    SetDoctype(SetDoctypePayload),
    InsertElement(InsertElementPayload),
    RemoveElement(RemoveElementPayload),
    SetElementName(SetElementNamePayload),
    SetAttribute(SetAttributePayload),
    SetText(SetTextPayload),
    SetViewBox(SetViewBoxPayload),
    SetTransform(SetTransformPayload),
}

pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`, or `None` when `next` differs in a part no leaf addresses (prolog or
/// epilog nodes, the presence of the root, or a root that is not an element). The root's attributes keep their order and the
/// children past the longest equal prefix are removed and reinserted.
pub fn net_mutations(base: &SvgSnapshot, next: &SvgSnapshot) -> Option<Vec<SvgMutation>> {
    if base.doc.prolog != next.doc.prolog || base.doc.epilog != next.doc.epilog {
        return None;
    }
    let mut leaves = Vec::new();
    if base.doc.declaration != next.doc.declaration {
        leaves.push(SvgMutation::SetDeclaration(SetDeclarationPayload { declaration: next.doc.declaration.clone() }));
    }
    if base.doc.doctype != next.doc.doctype {
        leaves.push(SvgMutation::SetDoctype(SetDoctypePayload { doctype: next.doc.doctype.clone() }));
    }
    match (&base.doc.root, &next.doc.root) {
        (None, None) => {}
        (Some(SvgNode::Element { name: base_name, attrs: base_attrs, children: base_children }), Some(SvgNode::Element { name, attrs, children })) => {
            if base_name != name {
                leaves.push(SvgMutation::SetElementName(SetElementNamePayload { path: Vec::new(), name: name.clone() }));
            }
            leaves.extend(net_attributes(base_attrs, attrs));
            let common = base_children.iter().zip(children).take_while(|(left, right)| left == right).count();
            leaves.extend((common..base_children.len()).rev().map(|index| SvgMutation::RemoveElement(RemoveElementPayload { parent: Vec::new(), index })));
            leaves.extend(children.iter().enumerate().skip(common).map(|(index, node)| SvgMutation::InsertElement(InsertElementPayload { parent: Vec::new(), index, node: node.clone() })));
        }
        _ => return None,
    }
    Some(leaves)
}

fn net_attributes(base: &[SvgAttr], next: &[SvgAttr]) -> Vec<SvgMutation> {
    let set = |name: &str, value, index| SvgMutation::SetAttribute(SetAttributePayload { path: Vec::new(), name: name.to_string(), value, index });
    let kept_in_order = base.iter().filter(|attribute| next.iter().any(|other| other.name == attribute.name)).map(|attribute| &attribute.name).eq(next.iter().filter(|attribute| base.iter().any(|other| other.name == attribute.name)).map(|attribute| &attribute.name));
    let mut leaves: Vec<SvgMutation> = base.iter().filter(|attribute| !kept_in_order || !next.iter().any(|other| other.name == attribute.name)).map(|attribute| set(&attribute.name, None, None)).collect();
    for (index, attribute) in next.iter().enumerate() {
        match base.iter().find(|other| other.name == attribute.name).filter(|_| kept_in_order) {
            Some(current) if current.value == attribute.value => {}
            Some(_) => leaves.push(set(&attribute.name, Some(attribute.value.clone()), None)),
            None => leaves.push(set(&attribute.name, Some(attribute.value.clone()), Some(index))),
        }
    }
    leaves
}
//#endregion 🔖️Net

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SvgMutation> {
    use crate::schema::snapshot::SvgNode;
    vec![
        SvgMutation::SetDeclaration(SetDeclarationPayload { declaration: None }),
        SvgMutation::SetDoctype(SetDoctypePayload { doctype: None }),
        SvgMutation::InsertElement(InsertElementPayload { parent: Vec::new(), index: 0, node: SvgNode::Text { text: "inserted".into() } }),
        SvgMutation::RemoveElement(RemoveElementPayload { parent: Vec::new(), index: 0 }),
        SvgMutation::SetElementName(SetElementNamePayload { path: Vec::new(), name: "svg".into() }),
        SvgMutation::SetAttribute(SetAttributePayload { path: Vec::new(), name: "attribute".into(), value: Some(crate::schema::snapshot::SvgAttributeValue::Text("value".into())), index: None }),
        SvgMutation::SetText(SetTextPayload { path: Vec::new(), text: "text".into() }),
        SvgMutation::SetViewBox(SetViewBoxPayload { path: Vec::new(), view_box: None, index: None }),
        SvgMutation::SetTransform(SetTransformPayload { path: Vec::new(), transform: None, index: None }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
