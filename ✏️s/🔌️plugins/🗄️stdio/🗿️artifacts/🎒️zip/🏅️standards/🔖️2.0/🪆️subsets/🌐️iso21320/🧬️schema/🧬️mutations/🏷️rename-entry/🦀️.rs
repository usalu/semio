//! 🏷️ `rename-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameEntry {
    pub(crate) name: String,
    pub(crate) new_name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for RenameEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "entry", kind: "rename-entry", record: "RenameEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { name, new_name } = self;
        {
            if base.entries.iter().any(|existing| existing.name == *new_name) {
                return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("a member named {new_name:?} already exists"), [new_name.clone()]);
            }
            protocol::MutationOutcome::new(diff::diff_rename_entry(name, new_name))
        }
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipIso21320Mutation>, semio_framework_value::ValueError> {
        let Self { name, new_name } = self;
        Ok(vec![ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name: new_name.clone(), new_name: name.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rename entry", "Eintrag umbenennen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
