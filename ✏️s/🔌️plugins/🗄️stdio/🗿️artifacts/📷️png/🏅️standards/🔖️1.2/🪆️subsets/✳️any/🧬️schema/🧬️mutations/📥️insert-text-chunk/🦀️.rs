//! 🧬️ Authoritative insert-text-chunk mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertTextChunkMutation {
    pub index: usize,
    pub chunk: PngTextChunk,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for InsertTextChunkMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "text-chunk", kind: "insert-text-chunk", record: "InsertTextChunk" };
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
            vec![PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: (*index).min(base.text_chunks.len()) })]
        }
    }
    fn label(&self) -> String {
        "insert text chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-text-chunk".into()]
    }
}
pub fn contribute(base: &PngSnapshot, index: usize, chunk: PngTextChunk) -> PngDiff {
    let at = index.min(base.text_chunks.len());
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![], modified: vec![], added: vec![PngTextChunkAdded { index: at, chunk }] }), chunk_order: Some(chunk_order_insert_text_diff(&base.chunk_order, at)), ..Default::default() }
}
pub fn chunk_order_insert_text_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Text { index } if *index >= at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Text { index: index + 1 } }),
            _ => None,
        })
        .collect();
    let added = vec![PngChunkOrderAdded { index: chunk_order_insert_pos(order, &PngChunkMarker::Text { index: at }), marker: PngChunkMarker::Text { index: at } }];
    PngChunkOrderDiff { removed: vec![], modified, added }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/🟣️direct-behavior/🦠️mutation/🔣️.json")).expect("committed insert-text-chunk payload")
}
#[cfg(test)]
#[path = "🧪️tests/🟣️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
