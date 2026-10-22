//! 🪧️ Authoritative PDF/UA mutation for set display doc title.

use crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::RemoveDisplayDocTitle;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDisplayDocTitle {
    pub display: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetDisplayDocTitle
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetDisplayDocTitle> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::RemoveDisplayDocTitle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "display-doc-title", kind: "set-display-doc-title", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/UA display document title to {}", self.display), &format!("PDF/UA-Anzeige des Dokumenttitels auf {} setzen", self.display))
    }

    fn target(&self) -> Vec<String> {
        vec!["ViewerPreferences.DisplayDocTitle".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetDisplayDocTitle
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetDisplayDocTitle> + From<crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::RemoveDisplayDocTitle>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::set_catalog_entry_rows(base, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(self.display)), self.entry_index)))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle") {
                Some(display) => vec![Op::from(SetDisplayDocTitle { display, entry_index: None })],
                None => vec![Op::from(RemoveDisplayDocTitle {})],
            }
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
