//! 🏷️ `set-tiny-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTinyAttribute {
    pub(crate) path: NodePath,
    pub(crate) name: String,
    pub(crate) value: Option<SvgAttributeValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for SetTinyAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "tiny-attribute", kind: "set-tiny-attribute", record: "SetTinyAttribute" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, name, value, index } = self;
        {
            if is_blocked_attribute(name) {
                return protocol::MutationOutcome::error(CODE_REJECTED, format!("attribute '{name}' is forbidden anywhere in SVG Tiny 1.1"), Vec::<String>::new());
            }
            protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[(name.as_str(), value.clone(), *index)]))
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let Self { path, name, .. } = self;
        let (value, index) = prior_attribute(base, path, name);
        Ok(vec![SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: path.clone(), name: name.clone(), value, index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set tiny attribute", "Tiny-Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
