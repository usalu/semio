//! 📥️ `insert-row` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertRow {
    pub(crate) element_name: String,
    pub(crate) index: usize,
    pub(crate) row: PlyRow,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertRow" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { element_name, index, row } = self;
        protocol::MutationOutcome::new(diff_insert_row(element_name, *index, row.clone()))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        let Self { element_name, index, .. } = self;
        Ok({
            {
                let at = base.elements.iter().find(|e| &e.name == element_name).map_or(*index, |e| (*index).min(e.rows.len()));
                vec![PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: element_name.clone(), index: at })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert row", "Zeile einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
