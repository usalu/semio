//! 🗣️ Authoritative PDF/UA mutation for set lang.

use super::remove_lang::RemoveLang;
use super::PdfUaMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetLang {
    pub lang: String,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetLang {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "lang", kind: "set-lang", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_catalog_entry(&mut next, "Lang", support::literal(&self.lang));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        match support::catalog_entry(base, "Lang") {
            Some(PdfObject::Str(bytes)) => vec![PdfUaMutation::SetLang(SetLang { lang: String::from_utf8_lossy(bytes).into_owned() })],
            _ => vec![PdfUaMutation::RemoveLang(RemoveLang {})],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/UA language to {}", self.lang)
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
        let mutation = SetLang { lang: "de-DE".to_string() };
        let outcome = <SetLang as MutationKind<PdfSnapshot, PdfUaMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(support::catalog_entry(&next, "Lang"), Some(&support::literal("de-DE")));
        assert_eq!(<SetLang as MutationKind<PdfSnapshot, PdfUaMutation>>::inverse(&mutation, &base), vec![PdfUaMutation::RemoveLang(RemoveLang {})]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
