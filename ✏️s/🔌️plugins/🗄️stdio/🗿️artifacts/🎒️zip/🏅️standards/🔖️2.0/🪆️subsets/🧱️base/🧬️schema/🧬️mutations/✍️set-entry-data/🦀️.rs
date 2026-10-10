//! ✍️ `set-entry-data` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-entry-data")]
pub struct SetEntryData {
    pub(crate) name: String,
    #[dsl(base64)]
    pub(crate) data: Vec<u8>,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for SetEntryData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entry-data", kind: "set-entry-data", record: "SetEntryData" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        let Self { name, data } = self;
        protocol::MutationOutcome::new(diff::diff_set_entry_data(name, data.clone()))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            {
                base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipMutation::SetEntryData(set_entry_data::SetEntryData { name: name.clone(), data: entry.data.clone() })]).unwrap_or_default()
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entry data", "Daten des Eintrags setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
