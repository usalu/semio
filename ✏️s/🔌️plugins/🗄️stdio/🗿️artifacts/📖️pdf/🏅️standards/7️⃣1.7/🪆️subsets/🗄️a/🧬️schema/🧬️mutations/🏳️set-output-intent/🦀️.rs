//! 🏳️ Authoritative PDF/A mutation for installing the required output intent.

use crate::standards::v1_7::subsets::a::schema::mutations::remove_output_intent::RemoveOutputIntent;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFA1";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = true;

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOutputIntent {
    pub identifier: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetOutputIntent
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetOutputIntent> + From<crate::standards::v1_7::subsets::a::schema::mutations::remove_output_intent::RemoveOutputIntent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "output-intent", kind: "set-output-intent", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::output_intent_rows(base, OUTPUT_INTENT_SUBTYPE, &self.identifier, OUTPUT_INTENT_DEST_PROFILE, &self.placements, self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            match support::output_intent_identifier(base) {
                Some(identifier) => vec![Op::from(SetOutputIntent { identifier, placements: support::placements_of(base, &support::output_intent_creation_ids(base)), entry_index: None })],
                None => vec![Op::from(RemoveOutputIntent {})],
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/A output intent \"{}\"", self.identifier), &format!("PDF/A-Ausgabebedingung \"{}\" setzen", self.identifier))
    }

    fn target(&self) -> Vec<String> {
        vec![self.identifier.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests
