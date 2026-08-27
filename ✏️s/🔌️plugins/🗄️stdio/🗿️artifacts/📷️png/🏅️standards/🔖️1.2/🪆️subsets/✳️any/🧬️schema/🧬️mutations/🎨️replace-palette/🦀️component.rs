//! 🧬️ Authoritative replace-palette mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePaletteMutation {
    pub plte: Option<Vec<PngRgb>>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ReplacePaletteMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "palette", kind: "replace-palette", record: "ReplacePalette" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { plte } = self;
        protocol::MutationOutcome::new(contribute(base, plte))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { plte } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ReplacePalette(crate::artifacts::png::schema::mutations::ReplacePaletteMutation { plte: base.plte.clone() })]
    }
    fn label(&self) -> String {
        "replace palette".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-palette".into()]
    }
}
pub fn contribute(base: &PngSnapshot, plte: &Option<Vec<PngRgb>>) -> PngDiff {
    PngDiff { plte: between_plte(&base.plte, plte), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Plte), PngChunkMarker::Plte, base.plte.is_some(), plte.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed replace-palette payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
