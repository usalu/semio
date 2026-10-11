//! 🤐️ Authoritative PDF/UA mutation for remove lang.

use super::set_lang::SetLang;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveLang {}

impl MutationKind<PdfSnapshot, PdfUaMutation> for RemoveLang {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "lang", kind: "remove-lang", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_rows(base, "Lang")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            let entry_index = support::catalog_entry_position(base, "Lang");
            match support::catalog_entry(base, "Lang") {
                Some(PdfObject::Str(bytes)) => vec![PdfUaMutation::SetLang(SetLang { lang: String::from_utf8_lossy(bytes).into_owned(), entry_index })],
                Some(PdfObject::Text(lang)) => vec![PdfUaMutation::SetLang(SetLang { lang: lang.clone(), entry_index })],
                _ => Vec::new(),
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/UA language", "PDF/UA-Sprache entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["Lang".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
