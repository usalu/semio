//! 🎛️ `set-track-codec` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-track-codec")]
#[value(rename_all = "camelCase")]
pub struct SetTrackCodec {
    pub track_index: usize,
    #[dsl(block)]
    pub codec: Mp4Codec,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetTrackCodec {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "track-codec", kind: "set-track-codec", record: "SetTrackCodec" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { track_index, codec } = self;
        protocol::MutationOutcome::new(track_diff_for(*track_index, Mp4TrackDiff { codec: Some(codec.clone()), ..Mp4TrackDiff::default() }))
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { track_index, .. } = self;
        Ok({
            match base.tracks.get(*track_index) {
                Some(track) => vec![Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index: *track_index, codec: track.codec.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set track codec", "Codec der Spur setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
