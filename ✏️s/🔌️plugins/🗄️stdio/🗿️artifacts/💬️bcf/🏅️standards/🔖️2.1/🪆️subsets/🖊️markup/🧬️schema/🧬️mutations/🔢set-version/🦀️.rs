//! 🔢️ `set-version` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetVersion {
    pub(crate) version: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetVersion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "version", kind: "set-version", record: "SetVersion" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { version } = self;
        protocol::MutationOutcome::new(BcfDiff { version: Some(version.clone()), topics: None, parts: None })
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        Ok(vec![BcfMutation::SetVersion(set_version::SetVersion { version: base.version.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set version", "Version setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
