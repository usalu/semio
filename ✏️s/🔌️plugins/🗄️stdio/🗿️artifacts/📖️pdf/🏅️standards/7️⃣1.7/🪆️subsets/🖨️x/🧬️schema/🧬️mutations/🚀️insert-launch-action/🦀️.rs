//! 🚀️ Authoritative PDF/X mutation for inserting a launch action.

use super::remove_launch_action::RemoveLaunchAction;
use super::PdfXMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertLaunchAction {
    pub target: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for InsertLaunchAction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "launch-action", kind: "insert-launch-action", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::action_object("Launch", "F", &self.target), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<PdfXMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfXMutation::RemoveLaunchAction(RemoveLaunchAction { target: self.target.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert launch action", "Launch-Aktion einfügen")
    }

    fn target(&self) -> Vec<String> {
        vec![self.target.clone()]
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
