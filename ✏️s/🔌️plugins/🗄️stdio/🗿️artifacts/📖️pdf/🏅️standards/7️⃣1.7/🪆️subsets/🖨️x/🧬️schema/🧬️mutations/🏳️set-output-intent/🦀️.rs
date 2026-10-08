//! 🏳️ Authoritative PDF/X mutation for installing the required output intent.

use super::remove_output_intent::RemoveOutputIntent;
use super::PdfXMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFX";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = true;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOutputIntent {
    pub identifier: String,
}

impl MutationKind<PdfSnapshot, PdfXMutation> for SetOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "output-intent", kind: "set-output-intent", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::output_intent_rows(base, OUTPUT_INTENT_SUBTYPE, &self.identifier, OUTPUT_INTENT_DEST_PROFILE)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfXMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match support::output_intent_identifier(base) {
            Some(identifier) => vec![PdfXMutation::SetOutputIntent(SetOutputIntent { identifier })],
            None => vec![PdfXMutation::RemoveOutputIntent(RemoveOutputIntent {})],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/X output intent \"{}\"", self.identifier), &format!("PDF/X-Ausgabebedingung \"{}\" setzen", self.identifier))
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
