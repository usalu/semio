//! 📤️ `remove-record` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRecord {
    pub index: usize,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for RemoveRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "record", kind: "remove-record", record: "RemoveRecord" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as Mutation<CsvSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) })
    }
    fn inverse(&self, base: &CsvSnapshot) -> Result<Vec<CsvMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.records.get(*index) {
                Some(record) => vec![CsvMutation::InsertRecord(insert_record::InsertRecord { index: *index, record: record.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove record", "Datensatz entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
