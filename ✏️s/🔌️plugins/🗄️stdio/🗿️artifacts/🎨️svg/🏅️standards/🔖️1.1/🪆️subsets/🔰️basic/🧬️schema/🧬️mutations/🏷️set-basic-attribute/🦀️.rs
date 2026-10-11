//! 🏷️ `set-basic-attribute` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBasicAttribute {
    pub(crate) path: NodePath,
    pub(crate) name: String,
    pub(crate) value: Option<SvgAttributeValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for SetBasicAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "basic-attribute", kind: "set-basic-attribute", record: "SetBasicAttribute" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, name, value, index } = self;
        {
            if local_name(name) == "clip-path" {
                if let Some(id) = value.as_ref().and_then(|v|if let SvgAttributeValue::LocalReference(id)=v {Some(id.as_str())}else{None}) {
                    if let Err(message) = resolve_clip_path(base, id) {
                        return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new());
                    }
                }
            }
            protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[(name.as_str(), value.clone(), *index)]))
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { path, name, .. } = self;
        let (value, index) = prior_attribute(base, path, name);
        Ok(vec![SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path: path.clone(), name: name.clone(), value, index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set basic attribute", "Basic-Attribut setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
