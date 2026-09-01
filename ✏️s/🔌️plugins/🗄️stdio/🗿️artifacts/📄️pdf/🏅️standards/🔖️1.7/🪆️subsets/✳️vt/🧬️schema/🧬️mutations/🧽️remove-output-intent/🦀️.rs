//! 🧽️ Authoritative PDF/VT mutation for removing the catalog output intent.

use super::set_output_intent::SetOutputIntent;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveOutputIntent {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "output-intent", kind: "remove-output-intent", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "OutputIntents");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        support::output_intent_identifier(base)
            .map(|identifier| PdfVtMutation::SetOutputIntent(SetOutputIntent { identifier }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove PDF/VT output intent".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["OutputIntents".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::set_output_intent::{OUTPUT_INTENT_DEST_PROFILE, OUTPUT_INTENT_SUBTYPE};
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
    use protocol::MutationDiff;

    #[test]
    fn removes_the_catalog_output_intent() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        support::set_output_intent(&mut base, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE);
        let mutation = RemoveOutputIntent {};
        let outcome = <RemoveOutputIntent as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::output_intent_identifier(&next).is_none());
        assert_eq!(<RemoveOutputIntent as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).len(), 1);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
