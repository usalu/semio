//! 🔃️ `reorder-rows` — repositions one row within the sequence (never spatial — `SemioTableRow`
//! carries no position of its own, only sequence order).

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReorderRows {
    pub from: usize,
    pub to: usize,
}

impl<Op: SemioTableOp> protocol::MutationKind<SemioTableSnapshot, Op> for ReorderRows {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "rows", kind: "reorder-rows", record: "ReorderedRows" };

    fn diff(&self, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move row #{} to #{}", self.from, self.to), &format!("Zeile #{} nach #{} verschieben", self.from, self.to))
    }
    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}
//#endregion 🔖️Payload
