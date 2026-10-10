//! ➖️ `remove-entry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-entry")]
pub struct RemoveEntry {
    pub(crate) name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for RemoveEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entry", kind: "remove-entry", record: "RemoveEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff::diff_remove_entry(name))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        let Self { name } = self;
        Ok({
            base.entries
                .iter()
                .position(|entry| entry.name == *name)
                .map(|index| vec![ZipMutation::AddEntry(add_entry::AddEntry { entry: base.entries[index].clone(), before: base.entries.get(index + 1).map(|entry| entry.name.clone()) })])
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
