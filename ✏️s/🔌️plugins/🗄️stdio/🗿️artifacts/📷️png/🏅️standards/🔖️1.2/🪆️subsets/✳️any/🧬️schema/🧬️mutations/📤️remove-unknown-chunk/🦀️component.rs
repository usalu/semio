//! 🧬️ Authoritative remove-unknown-chunk mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveUnknownChunkMutation {
    pub index: usize,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for RemoveUnknownChunkMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "unknown-chunk", kind: "remove-unknown-chunk", record: "RemoveUnknownChunk" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(contribute(base, *index))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { index } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.unknown_chunks.get(*index) {
            Some(chunk) => vec![PngMutation::InsertUnknownChunk(crate::artifacts::png::schema::mutations::InsertUnknownChunkMutation { index: *index, chunk: chunk.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove unknown chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-unknown-chunk".into()]
    }
}
pub fn contribute(base: &PngSnapshot, index: usize) -> PngDiff {
    if index >= base.unknown_chunks.len() {
        return PngDiff::default();
    }
    PngDiff { unknown_chunks: Some(PngUnknownChunksDiff { removed: vec![index], modified: vec![], added: vec![] }), chunk_order: Some(chunk_order_remove_unknown_diff(&base.chunk_order, index)), ..Default::default() }
}
pub fn chunk_order_remove_unknown_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let removed: Vec<usize> = order.iter().position(|m| matches!(m, PngChunkMarker::Unknown { index } if *index == at)).into_iter().collect();
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Unknown { index } if *index > at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Unknown { index: index - 1 } }),
            _ => None,
        })
        .collect();
    PngChunkOrderDiff { removed, modified, added: vec![] }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    let vector: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️component.json")).expect("authored mutation vector");
    serde_json::from_value(vector["mutation"].clone()).expect("direct mutation payload")
}
#[cfg(test)]
#[path = "🧪️tests/🦀️component.rs"]
mod tests;
