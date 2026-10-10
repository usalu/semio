//! 🏷️ `rename-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-entry")]
pub struct RenameEntry {
    pub(crate) name: String,
    pub(crate) new_name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for RenameEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "entry", kind: "rename-entry", record: "RenameEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { name, new_name } = self;
        protocol::MutationOutcome::new(diff::diff_rename_entry(name, new_name))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        let Self { name, new_name } = self;
        Ok({ vec![ZipMutation::RenameEntry(rename_entry::RenameEntry { name: new_name.clone(), new_name: name.clone() })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rename entry", "Eintrag umbenennen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
