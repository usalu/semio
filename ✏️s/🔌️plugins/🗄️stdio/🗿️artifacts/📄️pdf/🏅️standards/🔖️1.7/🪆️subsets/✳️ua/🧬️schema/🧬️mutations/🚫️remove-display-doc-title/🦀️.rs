//! 🚫️ Authoritative PDF/UA mutation for remove display doc title.

use super::set_display_doc_title::SetDisplayDocTitle;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveDisplayDocTitle {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveDisplayDocTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "display-doc-title", kind: "remove-display-doc-title", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "ViewerPreferences");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle")
            .map(|display| PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove PDF/UA title display preference".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["ViewerPreferences".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_catalog_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        support::set_catalog_entry(&mut base, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(true)));
        let mutation = RemoveDisplayDocTitle {};
        let outcome = <RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "ViewerPreferences").is_none());
        assert_eq!(<RemoveDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display: true })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Facets
