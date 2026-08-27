//! 🧬️ Transparent SvgMutation aggregate.
use crate::artifacts::svg::schema::diff::SvgDiff;
use crate::artifacts::svg::SvgSnapshot;
use serde::{Deserialize, Serialize};

pub use super::set_declaration::{SetDeclarationMutation, SetDeclarationPayload};
pub use super::set_doctype::{SetDoctypeMutation, SetDoctypePayload};
pub use super::insert_element::{InsertElementMutation, InsertElementPayload};
pub use super::remove_element::{RemoveElementMutation, RemoveElementPayload};
pub use super::set_element_name::{SetElementNameMutation, SetElementNamePayload};
pub use super::set_attribute::{SetAttributeMutation, SetAttributePayload};
pub use super::set_text::{SetTextMutation, SetTextPayload};
pub use super::set_view_box::{SetViewBoxMutation, SetViewBoxPayload};
pub use super::set_transform::{SetTransformMutation, SetTransformPayload};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "s.stdio.svg")]
pub enum SvgMutation {
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
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) { *snapshot = next; }
    outcome
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SvgMutation> {
    use crate::artifacts::xml::schema::snapshot::XmlNode;
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
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    #[test]
    fn aggregate_roster_is_exact() { assert_eq!(SvgMutation::kinds().len(), 9); }
}
