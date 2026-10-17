//! 🏷️ Authoritative PDF/VT mutation for set dpart metadata.

use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata::RemoveDpartMetadata;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDpartMetadata {
    pub job: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetDpartMetadata
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetDpartMetadata> + From<crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata::RemoveDpartMetadata>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-metadata", kind: "set-dpart-metadata", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::dpart_job_rows(base, Some(&self.job), self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::dpart_job(base) {
                Some(job) => vec![Op::from(SetDpartMetadata { job, entry_index: None })],
                None => vec![Op::from(RemoveDpartMetadata {})],
            }
        })
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

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
