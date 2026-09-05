//! 🧬️ Authoritative change-byte-order mutation.
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeByteOrderMutation {
    pub byte_order: TiffByteOrder,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for ChangeByteOrderMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "byte-order", kind: "change-byte-order", record: "ChangeByteOrder" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { byte_order } = self;
        protocol::MutationOutcome::new(contribute(base, *byte_order))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let Self { byte_order } = self;
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: base.byte_order })]
    }
    fn label(&self) -> String {
        "change byte order".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-byte-order".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, byte_order: TiffByteOrder) -> TiffDiff {
    TiffDiff { byte_order: (base.byte_order != byte_order).then_some(byte_order), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    serde_json::from_str(include_str!("🧪️tests/🟣️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-byte-order payload")
}
#[cfg(test)]
#[path = "🧪️tests/🟣️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
