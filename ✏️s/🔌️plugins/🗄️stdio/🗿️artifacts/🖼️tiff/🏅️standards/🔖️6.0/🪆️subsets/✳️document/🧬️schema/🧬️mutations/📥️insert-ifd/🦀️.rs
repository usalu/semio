//! 🧬️ Authoritative insert-ifd mutation.
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertIfdMutation {
    pub index: usize,
    pub ifd: TiffIfd,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for InsertIfdMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "ifd", kind: "insert-ifd", record: "InsertIfd" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { index, ifd } = self;
        protocol::MutationOutcome::new(contribute(base, *index, ifd.clone()))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let Self { index, ifd } = self;
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: (*index).min(base.ifds.len()) })]
    }
    fn label(&self) -> String {
        "insert ifd".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-ifd".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, index: usize, ifd: TiffIfd) -> TiffDiff {
    let at = index.min(base.ifds.len());
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![], added: vec![TiffIfdAdded { index: at, ifd }] }), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️.json")).expect("committed insert-ifd payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
