//! ➕️ `insert-row` — places a new row at a FINAL-state index in the row sequence (an intrinsically
//! ordered, anonymous collection — no stable id on `SemioTableRow`).

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRow {
    pub index: usize,
    pub row: SemioTableRow,
}

impl<Op: SemioTableOp> protocol::MutationKind<SemioTableSnapshot, Op> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertedRow" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert row at #{}", self.index), &format!("Zeile an #{} einfügen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op: SemioTableOp> protocol::ColdMutationKind<SemioTableSnapshot, Op> for InsertRow {
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
