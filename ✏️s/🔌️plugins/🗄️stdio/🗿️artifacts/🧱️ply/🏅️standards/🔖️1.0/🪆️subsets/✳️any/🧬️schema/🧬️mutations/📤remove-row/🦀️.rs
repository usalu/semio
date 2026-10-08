//! 📤️ `remove-row` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveRow {
    pub(crate) element_name: String,
    pub(crate) index: usize,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemoveRow" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { element_name, index } = self;
        protocol::MutationOutcome::new(diff_remove_row(element_name, *index))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { element_name, index } = self;
        Ok({
            match base.elements.iter().find(|e| &e.name == element_name).and_then(|e| e.rows.get(*index)) {
                Some(row) => vec![PlyMutation::InsertRow(insert_row::InsertRow { element_name: element_name.clone(), index: *index, row: row.clone() })],
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
