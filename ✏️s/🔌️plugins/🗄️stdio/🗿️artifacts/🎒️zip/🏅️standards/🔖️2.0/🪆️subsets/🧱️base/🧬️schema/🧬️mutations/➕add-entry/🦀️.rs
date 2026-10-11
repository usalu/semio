//! ➕️ `add-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "add-entry")]
pub struct AddEntry {
    #[dsl(block)]
    pub(crate) entry: ZipEntry,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) before: Option<String>,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for AddEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "entry", kind: "add-entry", record: "AddEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { entry, before } = self;
        protocol::MutationOutcome::new({
            if before.as_ref().is_some_and(|name| !base.entries.iter().any(|entry| &entry.name == name)) {
                return protocol::MutationOutcome::error("mutation.target-missing", "ZIP insertion anchor no longer exists", ["entries"]);
            }
            diff::diff_add_entry(base, entry.clone(), before.as_deref())
        })
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        let Self { entry, .. } = self;
        Ok({ vec![ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add entry", "Eintrag hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
