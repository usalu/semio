//! ➖️ `remove-track` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-track")]
pub struct RemoveTrack {
    pub index: usize,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for RemoveTrack {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "track", kind: "remove-track", record: "RemoveTrack" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }) })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.tracks.get(*index) {
                Some(track) => vec![Mp4Mutation::InsertTrack(insert_track::InsertTrack { index: *index, track: track.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove track", "Spur entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
