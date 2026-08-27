//! 🧬️ Authoritative replace-text-chunk mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceTextChunkMutation {
    pub index: usize,
    pub chunk: PngTextChunk,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ReplaceTextChunkMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "text-chunk", kind: "replace-text-chunk", record: "ReplaceTextChunk" };
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
        match base.text_chunks.get(*index) {
            Some(chunk) => vec![PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: *index, chunk: chunk.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "replace text chunk".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-text-chunk".into()]
    }
}
pub fn contribute(base: &PngSnapshot, index: usize, chunk: PngTextChunk) -> PngDiff {
    let existing = match base.text_chunks.get(index) {
        Some(c) => c,
        None => return PngDiff::default(),
    };
    let d = PngTextChunkDiff::between(existing, &chunk);
    if d.is_empty() {
        return PngDiff::default();
    }
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![], modified: vec![PngTextChunkModified { index, diff: d }], added: vec![] }), ..Default::default() }
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
