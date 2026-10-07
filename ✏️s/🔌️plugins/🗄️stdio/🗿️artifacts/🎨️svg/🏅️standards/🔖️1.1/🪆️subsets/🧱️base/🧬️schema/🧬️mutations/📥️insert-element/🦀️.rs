//! 🧬️ Direct insert-element mutation owner.
use crate::schema::diff::{diff_at_path, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::NodePath;
use crate::SvgSnapshot;
use crate::schema::snapshot::SvgNode;


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertElementPayload {
    pub parent: NodePath,
    pub index: usize,
    pub node: SvgNode,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum InsertElementMutation {
    Apply(InsertElementPayload),
    Restore(SvgDiff),
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for InsertElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "element", kind: "insert-element", record: "InsertedElement" };

    fn diff(&self, _base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(diff_at_path(
                &payload.parent,
                SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: payload.index, item: payload.node.clone() }] }) }),
            )),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<SvgSnapshot, super::SvgMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        let inverse = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::inverse(outcome.diff(), base);
        vec![super::SvgMutation::InsertElement(Self::Restore(inverse))]
    
    })())
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
