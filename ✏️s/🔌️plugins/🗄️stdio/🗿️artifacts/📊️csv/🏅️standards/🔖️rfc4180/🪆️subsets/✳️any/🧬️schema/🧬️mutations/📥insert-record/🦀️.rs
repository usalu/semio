//! 📥️ `insert-record` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRecord {
    pub index: usize,
    pub record: CsvRecord,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for InsertRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "record", kind: "insert-record", record: "InsertRecord" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as Mutation<CsvSnapshot>>::Diff> {
        let Self { index, record } = self;
        protocol::MutationOutcome::new({ CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CsvRecordAdded { index: *index, record: record.clone() }] }) } })
    }
    fn inverse(&self, base: &CsvSnapshot) -> Result<Vec<CsvMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            {
                vec![CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: *index })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert record", "Datensatz einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
