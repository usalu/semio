//! 🧽️ Authoritative PDF/VT mutation for removing the catalog output intent.

use crate::standards::v1_7::subsets::vt::schema::mutations::set_output_intent::SetOutputIntent;
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
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveOutputIntent> + From<crate::standards::v1_7::subsets::vt::schema::mutations::set_output_intent::SetOutputIntent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "output-intent", kind: "remove-output-intent", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_catalog_entry_owned_rows(base, "OutputIntents")))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::output_intent_identifier(base).map(|identifier| Op::from(SetOutputIntent { identifier, placements: support::placements_of(base, &support::output_intent_creation_ids(base)), entry_index: support::catalog_entry_position(base, "OutputIntents") })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove PDF/VT output intent", "PDF/VT-Ausgabebedingung entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec!["OutputIntents".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
