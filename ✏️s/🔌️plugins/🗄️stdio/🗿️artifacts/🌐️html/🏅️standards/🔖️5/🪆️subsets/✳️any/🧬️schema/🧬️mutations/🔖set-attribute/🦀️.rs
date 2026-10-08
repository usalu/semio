//! 🔖️ `set-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetAttribute {
    pub(crate) path: NodePath,
    pub(crate) name: String,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub(crate) value: Option<Option<String>>,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "attribute", kind: "set-attribute", record: "SetAttribute" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as Mutation<HtmlSnapshot>>::Diff> {
        let Self { path, name, value } = self;
        protocol::MutationOutcome::new(attribute_diff_at_path(base, path, name, value.clone()))
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Self { path, name, .. } = self;
        Ok(vec![HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set attribute", "Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
