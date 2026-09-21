//! 🏷️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-properties`.

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
pub struct SetProperties {
    pub properties: PdfNamedProperties,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetProperties {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "properties", kind: "set-properties", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_properties(base, self.properties.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.properties.iter().find(|item| item.name == self.properties.name) { Some(previous) => vec![PdfMutation::SetProperties(SetProperties { properties: previous.clone() })], None => vec![PdfMutation::RemoveProperties(super::remove_properties::RemoveProperties { name: self.properties.name.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set properties {}", self.properties.name), &format!("Eigenschaften {} setzen", self.properties.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.properties.name.clone()]
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
