//! ➖ `remove-row` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRow {
    pub(crate) index: usize,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemoveRow" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as Mutation<TsvSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(TsvDiff { records: Some(TsvRowsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..TsvDiff::default() })
    }
    fn inverse(&self, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.records.get(*index) {
                Some(row) => vec![TsvMutation::InsertRow(insert_row::InsertRow { index: *index, row: row.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove row", "Zeile entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
