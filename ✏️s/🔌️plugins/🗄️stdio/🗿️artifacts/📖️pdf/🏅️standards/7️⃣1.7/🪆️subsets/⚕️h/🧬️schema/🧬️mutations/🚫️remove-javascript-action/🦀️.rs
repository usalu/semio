//! 🚫️ Authoritative PDF/H mutation for removing a matching JavaScript action.

use crate::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action::InsertJavascriptAction;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveJavascriptAction {
    pub script: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveJavascriptAction
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveJavascriptAction> + From<crate::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action::InsertJavascriptAction>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "javascript-action", kind: "remove-javascript-action", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove JavaScript action", "JavaScript-Aktion entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec![self.script.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveJavascriptAction
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveJavascriptAction> + From<crate::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action::InsertJavascriptAction>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::action_with(base, "JavaScript", "JS", &self.script).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::action_with(base, "JavaScript", "JS", &self.script).map(|id| Op::from(InsertJavascriptAction { script: self.script.clone(), placements: support::placements_of(base, &[id]) })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
