//! 🤐️ Authoritative PDF/UA mutation for remove lang.

use super::set_lang::SetLang;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLang {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveLang {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "lang", kind: "remove-lang", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::remove_catalog_entry(&mut next, "Lang");
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        match support::catalog_entry(base, "Lang") {
            Some(PdfObject::Str(bytes)) => vec![PdfUaMutation::SetLang(SetLang { lang: String::from_utf8_lossy(bytes).into_owned() })],
            _ => Vec::new(),
        }
    }

    fn label(&self) -> String {
        "Remove PDF/UA language".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["Lang".to_string()]
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
        support::set_catalog_entry(&mut base, "Lang", support::literal("de-DE"));
        let mutation = RemoveLang {};
        let outcome = <RemoveLang as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::catalog_entry(&next, "Lang").is_none());
        assert_eq!(<RemoveLang as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
