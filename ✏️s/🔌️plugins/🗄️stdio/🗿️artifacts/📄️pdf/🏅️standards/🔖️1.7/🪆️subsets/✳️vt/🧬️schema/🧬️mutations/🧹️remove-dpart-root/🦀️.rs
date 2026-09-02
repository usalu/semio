//! 🧹️ Authoritative PDF/VT mutation for remove dpart root.

use super::set_dpart_root::SetDpartRoot;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
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
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        support::set_dpart_root(&mut base, "run 4711");
        let mutation = RemoveDpartRoot {};
        let outcome = <RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "DPartRoot").is_none());
        assert_eq!(<RemoveDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::SetDpartRoot(SetDpartRoot { job: "run 4711".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
