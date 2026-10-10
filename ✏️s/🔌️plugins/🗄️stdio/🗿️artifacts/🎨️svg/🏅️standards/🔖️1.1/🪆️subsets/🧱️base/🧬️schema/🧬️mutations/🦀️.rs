//! 🧬️ Transparent SvgMutation aggregate.
use crate::schema::diff::SvgDiff;
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

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

#[cfg(test)]
pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}


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
