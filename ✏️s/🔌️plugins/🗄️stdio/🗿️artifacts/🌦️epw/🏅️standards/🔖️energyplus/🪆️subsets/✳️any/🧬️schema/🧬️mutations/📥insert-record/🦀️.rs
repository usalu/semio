//! 📥️ `insert-record` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertRecord {
    pub index: usize,
    pub record: Box<EpwRecord>,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for InsertRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "record", kind: "insert-record", record: "InsertRecord" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as Mutation<EpwSnapshot>>::Diff> {
        let Self { index, record } = self;
        protocol::MutationOutcome::new({ EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![EpwRecordAdded { index: *index, record: record.as_ref().clone() }] }), ..EpwDiff::default() } })
    }
    fn inverse(&self, base: &EpwSnapshot) -> Result<Vec<EpwMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: *index })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert record", "Datensatz einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
