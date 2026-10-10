//! 🖼️ `set-view-box` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetViewBox {
    pub(crate) path: NodePath,
    pub(crate) view_box: Option<ViewBox>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for SetViewBox {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "view-box", kind: "set-view-box", record: "SetViewBox" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, view_box, index } = self;
        protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("viewBox", view_box.clone().map(SvgAttributeValue::ViewBox), *index)]))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "viewBox");
        Ok(vec![SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: path.clone(), view_box: value.and_then(|value| if let SvgAttributeValue::ViewBox(view_box) = value { Some(view_box) } else { None }), index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set view box", "ViewBox setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
