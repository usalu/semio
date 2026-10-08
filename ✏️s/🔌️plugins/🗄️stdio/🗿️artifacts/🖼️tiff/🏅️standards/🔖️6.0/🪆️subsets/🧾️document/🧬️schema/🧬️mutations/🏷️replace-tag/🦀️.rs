//! 🧬️ Authoritative replace-tag mutation.
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceTagMutation {
    pub ifd_index: usize,
    pub tag: u16,
    pub values: TiffValues,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for ReplaceTagMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "tag", kind: "replace-tag", record: "ReplaceTag" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { ifd_index, tag, values } = self;
        protocol::MutationOutcome::new(contribute(base, *ifd_index, *tag, values.clone()))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        let Self { ifd_index, tag, values } = self;
        Ok(match base.ifds.get(*ifd_index).map(|ifd| ifd.entries.iter().find(|entry| entry.tag == *tag)) {
            Some(Some(existing)) if existing.values == *values => Vec::new(),
            Some(Some(existing)) => vec![TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: *ifd_index, tag: *tag, values: existing.values.clone() })],
            Some(None) => vec![TiffMutation::RemoveTag(crate::schema::mutations::RemoveTagMutation { ifd_index: *ifd_index, tag: *tag })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace tag", "Tag ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-tag".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, ifd_index: usize, tag: u16, values: TiffValues) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    let already = ifd.entries.iter().find(|t| t.tag == tag);
    if let Some(existing) = already {
        if existing.values == values {
            return TiffDiff::default();
        }
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![], modified: vec![TiffTagModified { tag, values }], added: vec![] }, blocks: None, runs: vec![] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![], modified: vec![], added: vec![TiffTagAdded { tag, values }] }, blocks: None, runs: vec![] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    }
}
//#endregion Semantics


#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
