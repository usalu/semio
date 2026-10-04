//! 📸️ Total, invertible PDF snapshot replacement used by typed artifact editors.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: PdfSnapshot,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetSnapshot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(PdfDiff::between(base, &self.snapshot))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace PDF snapshot", "PDF-Snapshot ersetzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
