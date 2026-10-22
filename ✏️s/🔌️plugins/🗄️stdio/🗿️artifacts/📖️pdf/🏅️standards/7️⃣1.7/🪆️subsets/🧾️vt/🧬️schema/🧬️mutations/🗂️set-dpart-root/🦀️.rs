//! 🗂️ Authoritative PDF/VT mutation for set dpart root.

use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::RemoveDpartRoot;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDpartRoot {
    pub job: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetDpartRoot
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetDpartRoot> + From<crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::RemoveDpartRoot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-root", kind: "set-dpart-root", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/VT document partition {}", self.job), &format!("PDF/VT-Dokumentpartition {} setzen", self.job))
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetDpartRoot
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetDpartRoot> + From<crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::RemoveDpartRoot>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::dpart_root_rows(base, &self.job, &self.placements, self.entry_index)))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "DPartRoot") {
                Some(_) => vec![Op::from(SetDpartRoot { job: support::dpart_job(base).unwrap_or_default(), placements: support::placements_of(base, &support::dpart_root_creation_ids(base)), entry_index: None })],
                None => vec![Op::from(RemoveDpartRoot {})],
            }
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
