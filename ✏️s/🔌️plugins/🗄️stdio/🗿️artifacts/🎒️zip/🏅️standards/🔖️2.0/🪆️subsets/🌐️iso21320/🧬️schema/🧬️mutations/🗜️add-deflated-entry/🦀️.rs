//! 🗜️ `add-deflated-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddDeflatedEntry {
    pub(crate) entry: ZipEntry,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) before: Option<String>,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for AddDeflatedEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "deflated-entry", kind: "add-deflated-entry", record: "AddDeflatedEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { entry, before } = self;
        added_entry_diff(base, entry, before.as_deref(), ZipIso21320Method::Deflate)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipIso21320Mutation>, semio_framework_value::ValueError> {
        let Self { entry, .. } = self;
        Ok(vec![ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add deflated entry", "Komprimierten Eintrag hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
