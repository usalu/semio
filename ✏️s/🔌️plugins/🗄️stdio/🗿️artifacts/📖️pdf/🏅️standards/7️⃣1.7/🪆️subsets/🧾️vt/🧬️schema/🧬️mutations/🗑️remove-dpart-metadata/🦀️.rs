//! 🗑️ Authoritative PDF/VT mutation for remove dpart metadata.

use super::set_dpart_metadata::SetDpartMetadata;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDpartMetadata {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveDpartMetadata {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "dpart-metadata", kind: "remove-dpart-metadata", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::dpart_job_rows(base, None, None)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfVtMutation>, semio_framework_value::ValueError> {
        Ok({
            support::dpart_job(base).map(|job| PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job, entry_index: support::dpart_root_node(base).and_then(|node| support::entry_position(base, node, "DPM")) })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/VT partition metadata", "PDF/VT-Partitionsmetadaten entfernen")
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
