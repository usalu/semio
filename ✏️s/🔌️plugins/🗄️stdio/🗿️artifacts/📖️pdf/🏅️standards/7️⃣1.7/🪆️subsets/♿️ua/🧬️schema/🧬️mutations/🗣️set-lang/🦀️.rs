//! 🗣️ Authoritative PDF/UA mutation for set lang.

use crate::standards::v1_7::subsets::ua::schema::mutations::remove_lang::RemoveLang;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLang {
    pub lang: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetLang
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetLang> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_lang::RemoveLang>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "lang", kind: "set-lang", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/UA language to {}", self.lang), &format!("PDF/UA-Sprache auf {} setzen", self.lang))
    }

    fn target(&self) -> Vec<String> {
        vec!["Lang".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetLang
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetLang> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_lang::RemoveLang>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::set_catalog_entry_rows(base, "Lang", support::literal(&self.lang), self.entry_index)))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_entry(base, "Lang") {
                Some(PdfObject::Str(bytes)) => vec![Op::from(SetLang { lang: String::from_utf8_lossy(bytes).into_owned(), entry_index: None })],
                Some(PdfObject::Text(lang)) => vec![Op::from(SetLang { lang: lang.clone(), entry_index: None })],
                _ => vec![Op::from(RemoveLang {})],
            }
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
