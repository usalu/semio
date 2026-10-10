//! 🪧️ `stamp-base-profile` — authored as its own mutation leaf. It builds its own sparse diff and concrete
//! inverse from its payload and reads of `base`.
//!
//! `stamp` is not itself an approved semantic verb (`protocol::APPROVED_VERBS`); the operation it
//! performs is a plain attribute assignment on the root, so `SEMANTICS.verb` is `"set"`, matching
//! what `attributes_diff_at_path` actually does underneath.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct StampBaseProfile {
    pub(crate) base_profile: Option<String>,
    pub(crate) version: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) base_profile_index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) version_index: Option<usize>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for StampBaseProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "base-profile", kind: "stamp-base-profile", record: "StampBaseProfile" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { base_profile, version, base_profile_index, version_index } = self;
        protocol::MutationOutcome::new(attributes_diff_at_path(base, &[], &[("baseProfile", base_profile.clone().map(SvgAttributeValue::Text), *base_profile_index), ("version", version.clone().map(SvgAttributeValue::Text), *version_index)]))
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let text = |value: Option<SvgAttributeValue>| value.and_then(|value| value.text().map(str::to_owned));
        let (base_profile, base_profile_index) = prior_attribute(base, &[], "baseProfile");
        let (version, version_index) = prior_attribute(base, &[], "version");
        Ok(vec![SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: text(base_profile), version: text(version), base_profile_index, version_index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Stamp base profile", "Basisprofil kennzeichnen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
