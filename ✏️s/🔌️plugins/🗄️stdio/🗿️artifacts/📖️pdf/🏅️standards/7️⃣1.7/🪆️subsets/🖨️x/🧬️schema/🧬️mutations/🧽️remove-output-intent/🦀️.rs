//! 🧽️ Authoritative PDF/X mutation for removing the catalog output intent.

use super::set_output_intent::SetOutputIntent;
use super::PdfXMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveOutputIntent {}

impl MutationKind<PdfSnapshot, PdfXMutation> for RemoveOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "output-intent", kind: "remove-output-intent", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "OutputIntents");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfXMutation> {
        support::output_intent_identifier(base).map(|identifier| PdfXMutation::SetOutputIntent(SetOutputIntent { identifier })).into_iter().collect()
    }

    fn label(&self) -> String {
        "Remove PDF/X output intent".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["OutputIntents".to_string()]
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
