//! 🏷️ `set-version-info` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetVersionInfo {
    pub(crate) version: String,
    pub(crate) maintenance_version: u8,
    pub(crate) codepage: u16,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetVersionInfo {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "version-info", kind: "set-version-info", record: "SetVersionInfo" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as Mutation<DwgSnapshot>>::Diff> {
        let Self { version, maintenance_version, codepage } = self;
        let next = diff::version_info_next(base, version, *maintenance_version, *codepage);
        match crate::standards::v_ac1024::subsets::any::schema::snapshot::unwritable_version(&next) {
            Some((_, message)) => protocol::MutationOutcome::fatal("mutation.invariant", message, Vec::<String>::new()),
            None => protocol::MutationOutcome::new(diff::diff_set_version_info(base, version, *maintenance_version, *codepage)),
        }
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok(vec![DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: base.version.clone(), maintenance_version: base.maintenance_version, codepage: base.codepage })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set version info", "Versionsinfo setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
