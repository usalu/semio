//! 🏳️ Authoritative PDF/E mutation for installing the required output intent.

use super::remove_output_intent::RemoveOutputIntent;
use super::PdfEMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFX";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = false;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOutputIntent {
    pub identifier: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfEMutation> for SetOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "output-intent", kind: "set-output-intent", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::output_intent_rows(base, OUTPUT_INTENT_SUBTYPE, &self.identifier, OUTPUT_INTENT_DEST_PROFILE, &self.placements, self.entry_index)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfEMutation>, semio_framework_value::ValueError> {
        Ok({
            match support::output_intent_identifier(base) {
                Some(identifier) => vec![PdfEMutation::SetOutputIntent(SetOutputIntent { identifier, placements: Vec::new(), entry_index: None })],
                None => vec![PdfEMutation::RemoveOutputIntent(RemoveOutputIntent {})],
            }
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/E output intent \"{}\"", self.identifier), &format!("PDF/E-Ausgabebedingung \"{}\" setzen", self.identifier))
    }

    fn target(&self) -> Vec<String> {
        vec![self.identifier.clone()]
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
