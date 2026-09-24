//! 🎯️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-named-destination`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetNamedDestination {
    pub destination: PdfNamedDestination,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetNamedDestination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "named-destination", kind: "set-named-destination", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_named_destination(base, self.destination.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.named_destinations.iter().find(|item| item.name == self.destination.name) { Some(previous) => vec![PdfMutation::SetNamedDestination(SetNamedDestination { destination: previous.clone() })], None => vec![PdfMutation::RemoveNamedDestination(super::remove_named_destination::RemoveNamedDestination { name: self.destination.name.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set named destination {}", self.destination.name), &format!("Benanntes Ziel {} setzen", self.destination.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.destination.name.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
