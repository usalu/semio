//! 📸️ Own PDF 1.4 page snapshot replacement with frozen schema identity.
use super::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetSnapshot { pub snapshot: PdfSnapshot }
impl MutationKind<PdfSnapshot, PdfMutation> for SetSnapshot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if self.snapshot.schema != base.schema {
            let error = semio_s_artifact_stdio_contract::editing::SnapshotEditError::new("snapshot-edit.schema-identity", "/schema", "Snapshot replacement cannot change schema identity");
            return MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]);
        }

        MutationOutcome::new(PdfDiff::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
        if self.snapshot.schema != base.schema { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Snapshot replacement cannot change schema identity")); }
        Ok(vec![PdfMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace PDF pages", "PDF-Seiten ersetzen")
    }
    fn target(&self) -> Vec<String> { Vec::new() }
}
