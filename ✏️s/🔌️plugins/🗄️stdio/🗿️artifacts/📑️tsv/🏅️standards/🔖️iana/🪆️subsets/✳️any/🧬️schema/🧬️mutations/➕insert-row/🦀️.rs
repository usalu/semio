//! ➕ `insert-row` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRow {
    pub(crate) index: usize,
    pub(crate) row: Vec<String>,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertRow" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as Mutation<TsvSnapshot>>::Diff> {
        let Self { index, row } = self;
        protocol::MutationOutcome::new(TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![TsvRowAdded { index: *index, row: row.clone() }] }), ..TsvDiff::default() })
    }
    fn inverse(&self, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![TsvMutation::RemoveRow(remove_row::RemoveRow { index: *index })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert row", "Zeile einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
