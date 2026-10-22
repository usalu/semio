//! ✏️ `edit-cell` — replaces one cell's authored value, addressed by BASE-state `row_index` and
//! the column's `column_name` (`📓️taxonomy.md`'s `edit` row literally lists "cell" as an example:
//! "Replace an authored content body (text, cell, code)").

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct EditCell {
    pub row_index: usize,
    pub column_name: String,
    pub new_value: SemioValue,
}

impl<Op: SemioTableOp> protocol::MutationKind<SemioTableSnapshot, Op> for EditCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "cell", kind: "edit-cell", record: "EditedCell" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit cell #{} {}", self.row_index, self.column_name), &format!("Zelle #{} {} bearbeiten", self.row_index, self.column_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.row_index.to_string(), self.column_name.clone()]
    }
}

impl<Op: SemioTableOp> protocol::ColdMutationKind<SemioTableSnapshot, Op> for EditCell {
    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
