//! 🗂️ Authoritative PDF/VT mutation for set dpart root.

use super::remove_dpart_root::RemoveDpartRoot;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDpartRoot {
    pub job: String,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for SetDpartRoot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-root", kind: "set-dpart-root", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_dpart_root(&mut next, &self.job);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        vec![PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {})]
    }

    fn label(&self) -> String {
        format!("Set PDF/VT document partition {}", self.job)
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
        let mutation = SetDpartRoot { job: "run 4711".to_string() };
        let outcome = <SetDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "DPartRoot").is_some());
        assert_eq!(support::dpart_job(&next).as_deref(), Some("run 4711"));
        assert_eq!(<SetDpartRoot as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
