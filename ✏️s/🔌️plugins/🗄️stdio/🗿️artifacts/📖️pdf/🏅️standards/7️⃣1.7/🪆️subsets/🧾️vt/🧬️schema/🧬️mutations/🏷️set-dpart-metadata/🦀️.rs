//! 🏷️ Authoritative PDF/VT mutation for set dpart metadata.

use super::remove_dpart_metadata::RemoveDpartMetadata;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDpartMetadata {
    pub job: String,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for SetDpartMetadata {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-metadata", kind: "set-dpart-metadata", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::dpart_job_rows(base, Some(&self.job))))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfVtMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match support::dpart_job(base) {
            Some(job) => vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job })],
            None => vec![PdfVtMutation::RemoveDpartMetadata(RemoveDpartMetadata {})],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/VT partition metadata {}", self.job), &format!("PDF/VT-Partitionsmetadaten {} setzen", self.job))
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot.DPartRootNode.DPM".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
