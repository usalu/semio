//! 🧬️ Authoritative replace-tag mutation.
use crate::artifacts::tiff::schema::diff::{self, *};
use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceTagMutation {
    pub ifd_index: usize,
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for ReplaceTagMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "tag", kind: "replace-tag", record: "ReplaceTag" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { ifd_index, tag, kind, values } = self;
        protocol::MutationOutcome::new(contribute(base, *ifd_index, *tag, *kind, values.clone()))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let Self { ifd_index, tag, kind, values } = self;
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.ifds.get(*ifd_index) {
            Some(ifd) => match ifd.entries.iter().find(|t| t.tag == *tag) {
                Some(existing) => vec![TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() })],
                None => vec![TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: *ifd_index, tag: *tag })],
            },
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "replace tag".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-tag".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, ifd_index: usize, tag: u16, kind: TiffFieldType, values: TiffValues) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    let already = ifd.entries.iter().find(|t| t.tag == tag);
    if let Some(existing) = already {
        if existing.kind == kind && existing.values == values {
            return TiffDiff::default();
        }
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![], modified: vec![TiffTagModified { tag, kind, values }], added: vec![] }, pixels: None } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![], modified: vec![], added: vec![TiffTagAdded { tag, kind, values }] }, pixels: None } }],
                added: vec![],
            }),
            ..Default::default()
        }
    }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    serde_json::from_str(include_str!("🧪️tests/🟣️direct-behavior/🦠️mutation/🔣️.json")).expect("committed replace-tag payload")
}
#[cfg(test)]
#[path = "🧪️tests/🟣️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
