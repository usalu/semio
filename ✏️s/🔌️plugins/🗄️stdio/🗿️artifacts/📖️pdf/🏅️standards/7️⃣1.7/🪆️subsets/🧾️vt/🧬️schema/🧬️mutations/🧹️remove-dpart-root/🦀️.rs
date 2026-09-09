//! 🧹️ Authoritative PDF/VT mutation for remove dpart root.

use super::set_dpart_root::SetDpartRoot;
use super::PdfVtMutation;
#[cfg(test)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDpartRoot {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveDpartRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "dpart-root", kind: "remove-dpart-root", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "DPartRoot");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: support::dpart_job(base).unwrap_or_default() })]
    }

    fn label(&self) -> String {
        "Remove PDF/VT document partition".to_string()
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
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
