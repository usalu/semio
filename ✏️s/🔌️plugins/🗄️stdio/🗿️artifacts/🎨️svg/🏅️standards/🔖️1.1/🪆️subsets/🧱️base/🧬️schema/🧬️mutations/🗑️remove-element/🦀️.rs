//! 🧬️ Direct remove-element mutation owner.
use crate::schema::diff::{diff_at_path, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{node_at, NodePath, SvgNode};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveElementPayload {
    pub parent: NodePath,
    pub index: usize,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for RemoveElementPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "element", kind: "remove-element", record: "RemovedElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { parent, index } = self;
        protocol::MutationOutcome::new(diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { children: Some(SvgChildrenDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { parent, index } = self;
        Ok(match node_at(&base.doc, parent) {
            Ok(SvgNode::Element { children, .. }) => children.get(*index).map(|node| super::SvgMutation::InsertElement(super::InsertElementPayload { parent: parent.clone(), index: *index, node: node.clone() })).into_iter().collect(),
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove Element", "Element entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-element".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
