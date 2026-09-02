//! 🗑️ Authoritative PDF/VT mutation for remove dpart metadata.

use super::set_dpart_metadata::SetDpartMetadata;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDpartMetadata {}

impl MutationKind<PdfSnapshot, PdfVtMutation> for RemoveDpartMetadata {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "dpart-metadata", kind: "remove-dpart-metadata", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_dpart_job(&mut next, None);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        support::dpart_job(base)
            .map(|job| PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove PDF/VT partition metadata".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot.DPartRootNode.DPM".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        support::set_dpart_root(&mut base, "run 4711");
        let mutation = RemoveDpartMetadata {};
        let outcome = <RemoveDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::dpart_job(&next).is_none());
        assert_eq!(<RemoveDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "run 4711".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
