//! ✂️ `set-clip-path-reference` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::CODE_REJECTED;
#[allow(unused_imports)]
use crate::schema::snapshot::NodePath;
#[allow(unused_imports)]
use crate::schema::snapshot::SvgAttributeValue;
#[allow(unused_imports)]
use crate::schema::diff::SvgDiff;
#[allow(unused_imports)]
use crate::SvgSnapshot;
#[allow(unused_imports)]
use crate::schema::mutation_support::attributes_diff_at_path;
#[allow(unused_imports)]
use crate::schema::mutation_support::prior_attribute;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::support::resolve_clip_path;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::basic::schema::mutations::set_basic_attribute;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetClipPathReference {
    pub(crate) path: NodePath,
    pub(crate) clip_path_id: Option<String>,
}

impl<Op> protocol::MutationKind<SvgSnapshot, Op> for SetClipPathReference
where
    Op: protocol::Mutation<SvgSnapshot, Diff = SvgDiff> + From<SetClipPathReference> + From<crate::standards::v1_1::subsets::basic::schema::mutations::set_basic_attribute::SetBasicAttribute>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "clip-path-reference", kind: "set-clip-path-reference", record: "SetClipPathReference" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set clip path reference", "Referenz des Beschneidungspfads setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SvgSnapshot, Op> for SetClipPathReference
where
    Op: protocol::ColdMutation<SvgSnapshot, ReferenceDiff = SvgDiff> + From<SetClipPathReference> + From<crate::standards::v1_1::subsets::basic::schema::mutations::set_basic_attribute::SetBasicAttribute>,
{
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
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        let Self { path, .. } = self;
        let (value, index) = prior_attribute(base, path, "clip-path");
        Ok(vec![Op::from(set_basic_attribute::SetBasicAttribute { path: path.clone(), name: "clip-path".into(), value, index })])
    }
}
//#endregion 🔖️Payload
