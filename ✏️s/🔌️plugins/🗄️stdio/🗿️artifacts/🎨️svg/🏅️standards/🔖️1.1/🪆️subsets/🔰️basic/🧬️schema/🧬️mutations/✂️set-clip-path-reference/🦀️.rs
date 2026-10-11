//! ✂️ `set-clip-path-reference` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetClipPathReference {
    pub(crate) path: NodePath,
    pub(crate) clip_path_id: Option<String>,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for SetClipPathReference {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "clip-path-reference", kind: "set-clip-path-reference", record: "SetClipPathReference" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { path, clip_path_id } = self;
        {
            let value = match clip_path_id {
                Some(id) => match resolve_clip_path(base, id) {
                    Ok(_) => Some(SvgAttributeValue::LocalReference(id.clone())),
                    Err(message) => return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
                },
                None => None,
            };
            protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("clip-path", value, None)]))
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "clip-path");
        Ok(vec![SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path: path.clone(), name: "clip-path".into(), value, index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set clip path reference", "Referenz des Beschneidungspfads setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
