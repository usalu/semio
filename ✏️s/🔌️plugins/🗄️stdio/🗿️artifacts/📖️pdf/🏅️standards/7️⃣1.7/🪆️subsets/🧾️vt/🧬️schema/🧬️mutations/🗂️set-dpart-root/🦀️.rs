//! 🗂️ Authoritative PDF/VT mutation for set dpart root.

use super::remove_dpart_root::RemoveDpartRoot;
use super::PdfVtMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
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

impl MutationKind<PdfSnapshot, PdfVtMutation> for SetDpartRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-root", kind: "set-dpart-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::dpart_root_rows(base, &self.job, &self.placements, self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfVtMutation>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "DPartRoot") {
                Some(_) => vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: support::dpart_job(base).unwrap_or_default(), placements: support::placements_of(base, &support::dpart_root_creation_ids(base)), entry_index: None })],
                None => vec![PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {})],
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/VT document partition {}", self.job), &format!("PDF/VT-Dokumentpartition {} setzen", self.job))
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot".to_string()]
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
