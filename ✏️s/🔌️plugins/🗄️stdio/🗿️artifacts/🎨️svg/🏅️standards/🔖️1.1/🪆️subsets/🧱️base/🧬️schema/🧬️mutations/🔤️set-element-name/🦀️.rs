//! 🧬️ Direct set-element-name mutation owner.
use crate::schema::diff::{diff_at_path, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{node_at, NodePath, SvgNode};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetElementNamePayload {
    pub path: NodePath,
    pub name: String,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetElementNamePayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "element-name", kind: "set-element-name", record: "SetElementName" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, name } = self;
        protocol::MutationOutcome::new(diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: Some(name.clone()), ..Default::default() })))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        Ok(match node_at(&base.doc, path) {
            Ok(SvgNode::Element { name, .. }) => vec![super::SvgMutation::SetElementName(Self { path: path.clone(), name: name.clone() })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Element Name", "Elementname setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-element-name".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
