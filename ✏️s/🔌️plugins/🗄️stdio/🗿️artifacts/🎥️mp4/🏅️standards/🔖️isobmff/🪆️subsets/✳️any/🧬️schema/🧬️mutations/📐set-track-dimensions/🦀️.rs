//! 📐️ `set-track-dimensions` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-track-dimensions")]
#[value(rename_all = "camelCase")]
pub struct SetTrackDimensions {
    pub track_index: usize,
    pub width: u32,
    pub height: u32,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetTrackDimensions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "track-dimensions", kind: "set-track-dimensions", record: "SetTrackDimensions" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { track_index, width, height } = self;
        protocol::MutationOutcome::new(track_diff_for(*track_index, Mp4TrackDiff { width: Some(*width), height: Some(*height), ..Mp4TrackDiff::default() }))
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { track_index, .. } = self;
        Ok({
            match base.tracks.get(*track_index) {
                Some(track) => vec![Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index: *track_index, width: track.width, height: track.height })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set track dimensions", "Abmessungen der Spur setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
