//! 🧬️ Authoritative remove-tag mutation.
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveTagMutation {
    pub ifd_index: usize,
    pub tag: u16,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for RemoveTagMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "tag", kind: "remove-tag", record: "RemoveTag" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { ifd_index, tag } = self;
        protocol::MutationOutcome::new(contribute(base, *ifd_index, *tag))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        let Self { ifd_index, tag } = self;
        Ok(base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|entry| entry.tag == *tag)).map(|existing| TiffMutation::ReplaceTag(crate::schema::mutations::ReplaceTagMutation { ifd_index: *ifd_index, tag: *tag, values: existing.values.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove tag", "Tag entfernen")
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
        ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![TiffIfdModified { index: ifd_index, diff: TiffIfdDiff { entries: TiffTagsDiff { removed: vec![tag], modified: vec![], added: vec![] }, blocks: None, runs: vec![] } }], added: vec![] }),
        ..Default::default()
    }
}
//#endregion Semantics


#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
