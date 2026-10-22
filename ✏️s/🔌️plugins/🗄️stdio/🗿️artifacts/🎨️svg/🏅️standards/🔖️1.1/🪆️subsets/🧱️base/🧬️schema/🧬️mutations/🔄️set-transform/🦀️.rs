//! 🧬️ Direct set-transform mutation owner.
use crate::schema::diff::SvgDiff;
use crate::schema::mutation_support::{attribute_diff_at_path, prior_attribute};
use crate::schema::snapshot::{NodePath, SvgAttributeValue, TransformOp};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTransformPayload {
    pub path: NodePath,
    pub transform: Option<Vec<TransformOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for SetTransformPayload
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<SetTransformPayload>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "transform", kind: "set-transform", record: "SetTransform" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Transform", "Transformation setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-transform".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SvgSnapshot, Op> for SetTransformPayload
where
    Op: protocol::ColdMutation<SvgSnapshot, ReferenceDiff = SvgDiff> + From<SetTransformPayload>,
{
    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, transform, index } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, "transform", transform.clone().map(SvgAttributeValue::Transform), *index))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "transform");
        Ok(vec![Op::from(Self { path: path.clone(), transform: value.and_then(|value| if let SvgAttributeValue::Transform(transform) = value { Some(transform) } else { None }), index })])
    }
}


