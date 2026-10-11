//! 🔢️ `set-version` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 🔢️ Sets the LAS point format major/minor version.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetVersion {
    pub major: u8,
    pub minor: u8,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetVersion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "version", kind: "set-version", record: "SetVersion" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { major, minor } = self;
        protocol::MutationOutcome::new(diff::diff_set_version(*major, *minor))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetVersion(set_version::SetVersion { major: base.header.version_major, minor: base.header.version_minor })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set version", "Version setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
