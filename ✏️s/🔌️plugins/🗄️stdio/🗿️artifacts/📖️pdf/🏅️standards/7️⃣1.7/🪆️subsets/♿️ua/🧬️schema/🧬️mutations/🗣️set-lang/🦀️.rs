//! 🗣️ Authoritative PDF/UA mutation for set lang.

use super::remove_lang::RemoveLang;
use super::PdfUaMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLang {
    pub lang: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfUaMutation> for SetLang {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "lang", kind: "set-lang", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::set_catalog_entry_rows(base, "Lang", support::literal(&self.lang), self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfUaMutation>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "Lang") {
                Some(PdfObject::Str(bytes)) => vec![PdfUaMutation::SetLang(SetLang { lang: String::from_utf8_lossy(bytes).into_owned(), entry_index: None })],
                Some(PdfObject::Text(lang)) => vec![PdfUaMutation::SetLang(SetLang { lang: lang.clone(), entry_index: None })],
                _ => vec![PdfUaMutation::RemoveLang(RemoveLang {})],
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/UA language to {}", self.lang), &format!("PDF/UA-Sprache auf {} setzen", self.lang))
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
