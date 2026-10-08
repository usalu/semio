//! 🧬️ Direct set-attribute mutation owner.
use crate::schema::diff::SvgDiff;
use crate::schema::mutation_support::{attribute_diff_at_path, prior_attribute};
use crate::schema::snapshot::{NodePath, SvgAttributeValue};
use crate::SvgSnapshot;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAttributePayload {
    pub path: NodePath,
    pub name: String,
    pub value: Option<SvgAttributeValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetAttributePayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, name, value, index } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, name, value.clone(), *index))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { path, name, .. } = self;
        let (value, index) = prior_attribute(base, path, name);
        Ok(vec![super::SvgMutation::SetAttribute(Self { path: path.clone(), name: name.clone(), value, index })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Attribute", "Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-attribute".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
