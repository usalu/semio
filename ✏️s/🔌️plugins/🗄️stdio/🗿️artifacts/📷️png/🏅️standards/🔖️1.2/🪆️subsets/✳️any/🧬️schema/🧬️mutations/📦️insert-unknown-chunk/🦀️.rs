//! 🧬️ Authoritative insert-unknown-chunk mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertUnknownChunkMutation {
    pub index: usize,
    pub chunk: PngChunk,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for InsertUnknownChunkMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "unknown-chunk", kind: "insert-unknown-chunk", record: "InsertUnknownChunk" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { index, chunk } = self;
        protocol::MutationOutcome::new(contribute(base, *index, chunk.clone()))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { index, chunk } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![PngMutation::RemoveUnknownChunk(crate::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: (*index).min(base.unknown_chunks.len()) })]
        }
    }
    fn label(&self) -> String {
        "insert unknown chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-unknown-chunk".into()]
    }
}
pub fn contribute(base: &PngSnapshot, index: usize, chunk: PngChunk) -> PngDiff {
    let at = index.min(base.unknown_chunks.len());
    PngDiff { unknown_chunks: Some(PngUnknownChunksDiff { removed: vec![], modified: vec![], added: vec![PngUnknownChunkAdded { index: at, chunk }] }), chunk_order: Some(chunk_order_insert_unknown_diff(&base.chunk_order, at)), ..Default::default() }
}
pub fn chunk_order_insert_unknown_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Unknown { index } if *index >= at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Unknown { index: index + 1 } }),
            _ => None,
        })
        .collect();
    let added = vec![PngChunkOrderAdded { index: chunk_order_insert_pos(order, &PngChunkMarker::Unknown { index: at }), marker: PngChunkMarker::Unknown { index: at } }];
    PngChunkOrderDiff { removed: vec![], modified, added }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/🟣️direct-behavior/🦠️mutation/🔣️.json")).expect("committed insert-unknown-chunk payload")
}
#[cfg(test)]
#[path = "🧪️tests/🟣️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
