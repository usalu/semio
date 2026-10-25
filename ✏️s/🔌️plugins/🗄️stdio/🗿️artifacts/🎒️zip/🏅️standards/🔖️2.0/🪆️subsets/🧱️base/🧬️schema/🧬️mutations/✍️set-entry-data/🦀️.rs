//! ✍️ `set-entry-data` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, semio_framework_value::RetireOwned, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-entry-data")]
pub struct SetEntryData {
    pub name: String,
    #[dsl(base64)]
    pub data: Vec<u8>,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for SetEntryData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entry-data", kind: "set-entry-data", record: "SetEntryData" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set entry data", "Daten des Eintrags setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<ZipSnapshot, ZipMutation> for SetEntryData {
    fn diff(&self, _base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::ColdMutation<ZipSnapshot>>::ReferenceDiff> {
        let Self { name, data } = self;
        protocol::MutationOutcome::new(diff::diff_set_entry_data(name, data.clone()))
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            {
                base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipMutation::SetEntryData(SetEntryData { name: name.clone(), data: entry.data.clone() })]).unwrap_or_default()
            }
        })
    }
}
//#endregion 🔖️Payload
