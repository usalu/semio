//! 🎪️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-named-destination`.

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
pub struct RemoveNamedDestination {
    pub name: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveNamedDestination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "named-destination", kind: "remove-named-destination", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_named_destination(base, &self.name))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.named_destinations.iter().find(|item| item.name == self.name).map(|item| PdfMutation::SetNamedDestination(super::set_named_destination::SetNamedDestination { destination: item.clone() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove named destination {}", self.name), &format!("benannte Ziel {} entfernen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
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
