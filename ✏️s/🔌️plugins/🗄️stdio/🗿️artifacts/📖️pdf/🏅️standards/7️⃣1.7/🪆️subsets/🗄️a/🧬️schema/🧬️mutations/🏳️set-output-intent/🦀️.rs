//! 🏳️ Authoritative PDF/A mutation for installing the required output intent.

use super::remove_output_intent::RemoveOutputIntent;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFA1";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = true;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOutputIntent {
    pub identifier: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for SetOutputIntent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "output-intent", kind: "set-output-intent", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_output_intent(&mut next, OUTPUT_INTENT_SUBTYPE, &self.identifier, OUTPUT_INTENT_DEST_PROFILE);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        match support::output_intent_identifier(base) {
            Some(identifier) => vec![PdfAMutation::SetOutputIntent(SetOutputIntent { identifier })],
            None => vec![PdfAMutation::RemoveOutputIntent(RemoveOutputIntent {})],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/A output intent \"{}\"", self.identifier)
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
