//! 🧬️ Direct set-transform mutation owner.
use crate::schema::diff::SvgDiff;
use crate::schema::mutation_support::{attribute_diff_at_path, prior_attribute};
use crate::schema::snapshot::{NodePath, SvgAttributeValue, TransformOp};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTransformPayload {
    pub path: NodePath,
    pub transform: Option<Vec<TransformOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetTransformPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "transform", kind: "set-transform", record: "SetTransform" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, transform, index } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, "transform", transform.clone().map(SvgAttributeValue::Transform), *index))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "transform");
        Ok(vec![super::SvgMutation::SetTransform(Self { path: path.clone(), transform: value.and_then(|value| if let SvgAttributeValue::Transform(transform) = value { Some(transform) } else { None }), index })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Transform", "Transformation setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-transform".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
