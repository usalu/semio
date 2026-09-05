//! 🧬️ Authoritative remove-text-chunk mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveTextChunkMutation {
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
impl protocol::MutationKind<PngSnapshot, PngMutation> for RemoveTextChunkMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "text-chunk", kind: "remove-text-chunk", record: "RemoveTextChunk" };
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
        match base.text_chunks.get(*index) {
            Some(chunk) => vec![PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: *index, chunk: chunk.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove text chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-text-chunk".into()]
    }
}
pub fn contribute(base: &PngSnapshot, index: usize) -> PngDiff {
    if index >= base.text_chunks.len() {
        return PngDiff::default();
    }
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![index], modified: vec![], added: vec![] }), chunk_order: Some(chunk_order_remove_text_diff(&base.chunk_order, index)), ..Default::default() }
}
pub fn chunk_order_remove_text_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let removed: Vec<usize> = order.iter().position(|m| matches!(m, PngChunkMarker::Text { index } if *index == at)).into_iter().collect();
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Text { index } if *index > at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Text { index: index - 1 } }),
            _ => None,
        })
        .collect();
    PngChunkOrderDiff { removed, modified, added: vec![] }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed remove-text-chunk payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
