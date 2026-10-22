//! ➖️ `remove-row` — takes one row out of the sequence, addressed by BASE-state index.

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRow {
    pub index: usize,
}

impl<Op: SemioTableOp> protocol::MutationKind<SemioTableSnapshot, Op> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemovedRow" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove row #{}", self.index), &format!("Zeile #{} entfernen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op: SemioTableOp> protocol::ColdMutationKind<SemioTableSnapshot, Op> for RemoveRow {
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
