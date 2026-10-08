//! 🔲 `set-cell` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCell {
    pub(crate) row_index: usize,
    pub(crate) field_index: usize,
    pub(crate) value: String,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for SetCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "cell", kind: "set-cell", record: "SetCell" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as Mutation<TsvSnapshot>>::Diff> {
        let Self { row_index, field_index, value } = self;
        protocol::MutationOutcome::new({
            let mut fields = vec![None; field_index + 1];
            fields[*field_index] = Some(value.clone());
            TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: vec![TsvRowModified { index: *row_index, diff: TsvRowDiff { fields: Some(fields) } }], added: Vec::new() }), ..TsvDiff::default() }
        })
    }
    fn inverse(&self, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
        let Self { row_index, field_index, .. } = self;
        Ok({
            match base.records.get(*row_index).and_then(|r| r.get(*field_index)) {
                Some(cell) => vec![TsvMutation::SetCell(set_cell::SetCell { row_index: *row_index, field_index: *field_index, value: cell.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set cell", "Zelle setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
