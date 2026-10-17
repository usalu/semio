//! 📜️ Authoritative PDF/VT mutation for inserting a JavaScript action.

use crate::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action::RemoveJavascriptAction;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertJavascriptAction {
    pub script: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertJavascriptAction
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertJavascriptAction> + From<crate::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action::RemoveJavascriptAction>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "javascript-action", kind: "insert-javascript-action", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::action_object("JavaScript", "JS", &self.script), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(RemoveJavascriptAction { script: self.script.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert JavaScript action", "JavaScript-Aktion einfügen")
    }

    fn target(&self) -> Vec<String> {
        vec![self.script.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
