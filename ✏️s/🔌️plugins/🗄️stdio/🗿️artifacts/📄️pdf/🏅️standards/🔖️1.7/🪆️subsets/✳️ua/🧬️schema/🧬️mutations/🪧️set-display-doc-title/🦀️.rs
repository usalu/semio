//! 🪧️ Authoritative PDF/UA mutation for set display doc title.

use super::remove_display_doc_title::RemoveDisplayDocTitle;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDisplayDocTitle {
    pub display: bool,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetDisplayDocTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "display-doc-title", kind: "set-display-doc-title", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_catalog_entry(&mut next, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(self.display)));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        match support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle") {
            Some(display) => vec![PdfUaMutation::SetDisplayDocTitle(SetDisplayDocTitle { display })],
            None => vec![PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {})],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/UA display document title to {}", self.display)
    }

    fn target(&self) -> Vec<String> {
        vec!["ViewerPreferences.DisplayDocTitle".to_string()]
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
        let mutation = SetDisplayDocTitle { display: true };
        let outcome = <SetDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(support::catalog_flag(&next, "ViewerPreferences", "DisplayDocTitle"), Some(true));
        assert_eq!(<SetDisplayDocTitle as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveDisplayDocTitle(RemoveDisplayDocTitle {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
