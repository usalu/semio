//! 🔄️ `set-transform` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTransform {
    pub(crate) path: NodePath,
    pub(crate) transform: Option<Vec<TransformOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for SetTransform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "transform", kind: "set-transform", record: "SetTransform" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, transform, index } = self;
        protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("transform", transform.clone().map(SvgAttributeValue::Transform), *index)]))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "transform");
        Ok(vec![SvgTinyMutation::SetTransform(set_transform::SetTransform { path: path.clone(), transform: value.and_then(|value| if let SvgAttributeValue::Transform(transform) = value { Some(transform) } else { None }), index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set transform", "Transformation setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
