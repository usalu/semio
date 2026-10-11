//! ➖️ `remove-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntry {
    pub(crate) name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for RemoveEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entry", kind: "remove-entry", record: "RemoveEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff::diff_remove_entry(name))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipIso21320Mutation>, semio_framework_value::ValueError> {
        let Self { name } = self;
        Ok({
            base.entries
                .iter()
                .position(|entry| entry.name == *name)
                .map(|index| {
                    let entry = base.entries[index].clone();
                    let before = base.entries.get(index + 1).map(|entry| entry.name.clone());
                    if entry.metadata.compression_method == ZipIso21320Method::Stored.wire_code() {
                        vec![ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry, before })]
                    } else {
                        vec![ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry, before })]
                    }
                })
                .unwrap_or_default()
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entry", "Eintrag entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
