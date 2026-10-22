//! 🛬️ Authoritative PDF/X mutation for removing a matching launch action.

use crate::standards::v1_7::subsets::x::schema::mutations::insert_launch_action::InsertLaunchAction;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveLaunchAction {
    pub target: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveLaunchAction
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveLaunchAction> + From<crate::standards::v1_7::subsets::x::schema::mutations::insert_launch_action::InsertLaunchAction>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "launch-action", kind: "remove-launch-action", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove launch action", "Launch-Aktion entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec![self.target.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveLaunchAction
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveLaunchAction> + From<crate::standards::v1_7::subsets::x::schema::mutations::insert_launch_action::InsertLaunchAction>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::action_with(base, "Launch", "F", &self.target).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::action_with(base, "Launch", "F", &self.target).map(|id| Op::from(InsertLaunchAction { target: self.target.clone(), placements: support::placements_of(base, &[id]) })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
