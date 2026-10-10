//! 🧬️ Direct insert-element mutation owner.
use crate::schema::diff::{diff_at_path, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{NodePath, SvgNode};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertElementPayload {
    pub parent: NodePath,
    pub index: usize,
    pub node: SvgNode,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for InsertElementPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "element", kind: "insert-element", record: "InsertedElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index, node } = self;
        protocol::MutationOutcome::new(diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { children: Some(SvgChildrenDiff { added: vec![SvgChildAdded { index: *index, item: node.clone() }], ..Default::default() }), ..Default::default() })))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { parent, index, .. } = self;
        Ok(vec![super::SvgMutation::RemoveElement(super::RemoveElementPayload { parent: parent.clone(), index: *index })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert Element", "Element einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
