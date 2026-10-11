//! 🧬️ Direct set-view-box mutation owner.
use crate::schema::diff::SvgDiff;
use crate::schema::mutation_support::{attribute_diff_at_path, prior_attribute};
use crate::schema::snapshot::{NodePath, SvgAttributeValue, ViewBox};
use crate::SvgSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetViewBoxPayload {
    pub path: NodePath,
    pub view_box: Option<ViewBox>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetViewBoxPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "view-box", kind: "set-view-box", record: "SetViewBox" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, view_box, index } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, "viewBox", view_box.map(SvgAttributeValue::ViewBox), *index))
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "viewBox");
        Ok(vec![super::SvgMutation::SetViewBox(Self { path: path.clone(), view_box: value.and_then(|value| if let SvgAttributeValue::ViewBox(view_box) = value { Some(view_box) } else { None }), index })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set View Box", "ViewBox setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-view-box".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
