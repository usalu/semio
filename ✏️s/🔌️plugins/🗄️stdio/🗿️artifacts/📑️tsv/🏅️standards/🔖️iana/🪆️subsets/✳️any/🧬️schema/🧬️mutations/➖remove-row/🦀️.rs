//! ➖ `remove-row` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRow {
    pub index: usize,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemoveRow" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove row", "Zeile entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<TsvSnapshot, TsvMutation> for RemoveRow {
    fn diff(&self, _base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as protocol::ColdMutation<TsvSnapshot>>::ReferenceDiff> {
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
}
//#endregion 🔖️Payload
