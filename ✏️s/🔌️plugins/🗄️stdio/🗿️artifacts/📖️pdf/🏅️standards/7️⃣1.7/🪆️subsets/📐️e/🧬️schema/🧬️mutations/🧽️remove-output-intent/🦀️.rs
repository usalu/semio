//! 🧽️ Authoritative PDF/E mutation for removing the catalog output intent.

use crate::standards::v1_7::subsets::e::schema::mutations::set_output_intent::SetOutputIntent;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveOutputIntent {}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveOutputIntent
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveOutputIntent> + From<crate::standards::v1_7::subsets::e::schema::mutations::set_output_intent::SetOutputIntent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "output-intent", kind: "remove-output-intent", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/E output intent", "PDF/E-Ausgabebedingung entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["OutputIntents".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveOutputIntent
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveOutputIntent> + From<crate::standards::v1_7::subsets::e::schema::mutations::set_output_intent::SetOutputIntent>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "OutputIntents")))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::output_intent_identifier(base).map(|identifier| Op::from(SetOutputIntent { identifier, placements: support::placements_of(base, &support::output_intent_creation_ids(base)), entry_index: support::catalog_entry_position(base, "OutputIntents") })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
