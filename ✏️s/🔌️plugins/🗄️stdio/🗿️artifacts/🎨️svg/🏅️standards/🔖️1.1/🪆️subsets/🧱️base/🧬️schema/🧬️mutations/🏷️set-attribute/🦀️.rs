//! 🧬️ Direct set-attribute mutation owner.
use crate::schema::diff::SvgDiff;
use crate::schema::mutation_support::{attribute_diff_at_path, prior_attribute};
use crate::schema::snapshot::{NodePath, SvgAttributeValue};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAttributePayload {
    pub path: NodePath,
    pub name: String,
    pub value: Option<SvgAttributeValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for SetAttributePayload
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<SetAttributePayload>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Attribute", "Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-attribute".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SvgSnapshot, Op> for SetAttributePayload
where
    Op: protocol::ColdMutation<SvgSnapshot, ReferenceDiff = SvgDiff> + From<SetAttributePayload>,
{
    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, name, value, index } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, name, value.clone(), *index))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { path, name, .. } = self;
        let (value, index) = prior_attribute(base, path, name);
        Ok(vec![Op::from(Self { path: path.clone(), name: name.clone(), value, index })])
    }
}


