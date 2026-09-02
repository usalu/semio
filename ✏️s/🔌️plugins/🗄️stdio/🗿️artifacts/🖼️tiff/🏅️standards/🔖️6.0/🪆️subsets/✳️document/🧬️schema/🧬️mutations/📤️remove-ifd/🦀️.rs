//! 🧬️ Authoritative remove-ifd mutation.
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveIfdMutation {
    pub index: usize,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for RemoveIfdMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "ifd", kind: "remove-ifd", record: "RemoveIfd" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(contribute(base, *index))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let Self { index } = self;
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.ifds.get(*index) {
            Some(ifd) => vec![TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: *index, ifd: ifd.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove ifd".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-ifd".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, index: usize) -> TiffDiff {
    if index >= base.ifds.len() {
        return TiffDiff::default();
    }
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️.json")).expect("committed remove-ifd payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
