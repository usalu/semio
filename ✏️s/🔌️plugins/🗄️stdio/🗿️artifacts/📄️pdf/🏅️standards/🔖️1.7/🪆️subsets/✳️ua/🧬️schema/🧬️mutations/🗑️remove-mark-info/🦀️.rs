//! 🗑️ Authoritative PDF/UA mutation for remove mark info.

use super::set_mark_info::SetMarkInfo;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMarkInfo {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveMarkInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "mark-info", kind: "remove-mark-info", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "MarkInfo");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        support::catalog_flag(base, "MarkInfo", "Marked")
            .map(|marked| PdfUaMutation::SetMarkInfo(SetMarkInfo { marked }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        "Remove PDF/UA marked flag".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["MarkInfo".to_string()]
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
        support::set_catalog_entry(&mut base, "MarkInfo", support::single_entry_dict("Marked", PdfObject::Bool(true)));
        let mutation = RemoveMarkInfo {};
        let outcome = <RemoveMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "MarkInfo").is_none());
        assert_eq!(<RemoveMarkInfo as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetMarkInfo(SetMarkInfo { marked: true })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
