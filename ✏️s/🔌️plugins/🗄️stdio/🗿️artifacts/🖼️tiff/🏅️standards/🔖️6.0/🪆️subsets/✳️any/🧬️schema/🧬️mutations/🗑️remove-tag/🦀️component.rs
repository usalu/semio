//! 🧬️ Authoritative remove-tag mutation.
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveTagMutation {
    pub ifd_index: usize,
    pub tag: u16,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for RemoveTagMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "tag", kind: "remove-tag", record: "RemoveTag" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { ifd_index, tag } = self;
        protocol::MutationOutcome::new(contribute(base, *ifd_index, *tag))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let Self { ifd_index, tag } = self;
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|t| t.tag == *tag)) {
            Some(existing) => vec![TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove tag".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-tag".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, ifd_index: usize, tag: u16) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    if !ifd.entries.iter().any(|t| t.tag == tag) {
        return TiffDiff::default();
    }
    TiffDiff {
        ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![tag], modified: vec![], added: vec![] }, pixels: None } }], added: vec![] }),
        ..Default::default()
    }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    let vector: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️component.json")).expect("authored mutation vector");
    serde_json::from_value(vector["mutation"].clone()).expect("direct mutation payload")
}
#[cfg(test)]
#[path = "🧪️tests/🦀️component.rs"]
mod tests;
