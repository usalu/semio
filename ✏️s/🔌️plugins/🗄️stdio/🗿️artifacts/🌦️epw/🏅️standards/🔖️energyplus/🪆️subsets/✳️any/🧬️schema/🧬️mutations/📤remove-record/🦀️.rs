//! 📤️ `remove-record` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveRecord {
    pub index: usize,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for RemoveRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "record", kind: "remove-record", record: "RemoveRecord" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as Mutation<EpwSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(EpwDiff { records: Some(EpwRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..EpwDiff::default() })
    }
    fn inverse(&self, base: &EpwSnapshot) -> Result<Vec<EpwMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.records.get(*index) {
                Some(record) => vec![EpwMutation::InsertRecord(insert_record::InsertRecord { index: *index, record: Box::new(record.clone()) })],
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
