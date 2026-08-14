//! 🧬️ Mp4Mutation — named-variant vocabulary (imperative verbs, gif/svg precedent). Every
//! variant's `diff()` is handcrafted (constructs the sparse `Mp4Diff` directly — apply-and-capture
//! is banned); `inverse()` is handcrafted per variant, index-aware.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Box, Mp4Codec, Mp4Ftyp, Mp4Movie, Mp4Sample, Mp4Snapshot, Mp4Track, Mp4TrackMetadata};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::{IndexedAdded, IndexedDiff, IndexedModified, Mp4Diff, Mp4SampleDiff, Mp4TrackDiff};
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Mp4Mutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: Mp4Snapshot },
    SetFtyp { ftyp: Mp4Ftyp },
    InsertTrack { index: usize, track: Mp4Track },
    RemoveTrack { index: usize },
    SetTrackDimensions { track_index: usize, width: u32, height: u32 },
    SetTrackCodec { track_index: usize, codec: Mp4Codec },
    InsertSample { track_index: usize, index: usize, sample: Mp4Sample },
    RemoveSample { track_index: usize, index: usize },
    SetSampleSync { track_index: usize, index: usize, sync: bool },
    AddUnknownBox { index: usize, item: Mp4Box },
    RemoveUnknownBox { index: usize },
}

fn track_diff_for(track_index: usize, inner: Mp4TrackDiff) -> Mp4Diff {
    Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: track_index, diff: inner }], added: vec![] }), unknown_boxes: None }
}

fn sample_diff_for(track_index: usize, samples: IndexedDiff<Mp4Sample, Mp4SampleDiff>, chunk_sample_counts: Option<Vec<u32>>) -> Mp4Diff {
    track_diff_for(track_index, Mp4TrackDiff { samples: Some(samples), chunk_sample_counts, ..Mp4TrackDiff::default() })
}

impl Mutation<Mp4Snapshot> for Mp4Mutation {
    type Diff = Mp4Diff;

    fn diff(&self, base: &Mp4Snapshot) -> Self::Diff {
        match self {
            Mp4Mutation::NoMutation => Mp4Diff::default(),
            Mp4Mutation::SetSnapshot { snapshot } => <Mp4Diff as protocol::command::DiffAlgebra<Mp4Snapshot>>::between(base, snapshot),
            Mp4Mutation::SetFtyp { ftyp } => Mp4Diff { ftyp: Some(ftyp.clone()), movie: None, tracks: None, unknown_boxes: None },
            Mp4Mutation::InsertTrack { index, track } => {
                Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: track.clone() }] }), unknown_boxes: None }
            }
            Mp4Mutation::RemoveTrack { index } => {
                Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }), unknown_boxes: None }
            }
            Mp4Mutation::SetTrackDimensions { track_index, width, height } => track_diff_for(*track_index, Mp4TrackDiff { width: Some(*width), height: Some(*height), ..Mp4TrackDiff::default() }),
            Mp4Mutation::SetTrackCodec { track_index, codec } => track_diff_for(*track_index, Mp4TrackDiff { codec: Some(codec.clone()), ..Mp4TrackDiff::default() }),
            Mp4Mutation::InsertSample { track_index, index, sample } => {
                let count = base.tracks.get(*track_index).map_or(1, |track| track.samples.len() as u32 + 1);
                sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: sample.clone() }] }, Some(vec![count]))
            }
            Mp4Mutation::RemoveSample { track_index, index } => {
                let count = base.tracks.get(*track_index).map_or(0, |track| track.samples.len().saturating_sub(1) as u32);
                sample_diff_for(*track_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }, Some(vec![count]))
            }
            Mp4Mutation::SetSampleSync { track_index, index, sync } => {
                sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: Mp4SampleDiff { data: None, duration: None, cts_offset: None, sync: Some(*sync) } }], added: vec![] }, None)
            }
            Mp4Mutation::AddUnknownBox { index, item } => {
                Mp4Diff { ftyp: None, movie: None, tracks: None, unknown_boxes: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: item.clone() }] }) }
            }
            Mp4Mutation::RemoveUnknownBox { index } => Mp4Diff { ftyp: None, movie: None, tracks: None, unknown_boxes: Some(IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }) },
        }
    }

    fn inverse(&self, base: &Mp4Snapshot) -> Vec<Self> {
        match self {
            Mp4Mutation::NoMutation => vec![Mp4Mutation::NoMutation],
            Mp4Mutation::SetSnapshot { .. } => vec![Mp4Mutation::SetSnapshot { snapshot: base.clone() }],
            Mp4Mutation::SetFtyp { .. } => vec![Mp4Mutation::SetFtyp { ftyp: base.ftyp.clone() }],
            Mp4Mutation::InsertTrack { index, .. } => vec![Mp4Mutation::RemoveTrack { index: *index }],
            Mp4Mutation::RemoveTrack { index } => match base.tracks.get(*index) {
                Some(track) => vec![Mp4Mutation::InsertTrack { index: *index, track: track.clone() }],
                None => vec![Mp4Mutation::NoMutation],
            },
            Mp4Mutation::SetTrackDimensions { track_index, .. } => match base.tracks.get(*track_index) {
                Some(track) => vec![Mp4Mutation::SetTrackDimensions { track_index: *track_index, width: track.width, height: track.height }],
                None => vec![Mp4Mutation::NoMutation],
            },
            Mp4Mutation::SetTrackCodec { track_index, .. } => match base.tracks.get(*track_index) {
                Some(track) => vec![Mp4Mutation::SetTrackCodec { track_index: *track_index, codec: track.codec.clone() }],
                None => vec![Mp4Mutation::NoMutation],
            },
            Mp4Mutation::InsertSample { .. } | Mp4Mutation::RemoveSample { .. } => vec![Mp4Mutation::SetSnapshot { snapshot: base.clone() }],
            Mp4Mutation::SetSampleSync { track_index, index, .. } => match base.tracks.get(*track_index).and_then(|t| t.samples.get(*index)) {
                Some(sample) => vec![Mp4Mutation::SetSampleSync { track_index: *track_index, index: *index, sync: sample.sync }],
                None => vec![Mp4Mutation::NoMutation],
            },
            Mp4Mutation::AddUnknownBox { index, .. } => vec![Mp4Mutation::RemoveUnknownBox { index: *index }],
            Mp4Mutation::RemoveUnknownBox { index } => match base.unknown_boxes.get(*index) {
                Some(item) => vec![Mp4Mutation::AddUnknownBox { index: *index, item: item.clone() }],
                None => vec![Mp4Mutation::NoMutation],
            },
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention).
pub fn apply_mp4_mutation(snapshot: &mut Mp4Snapshot, mutation: &Mp4Mutation) -> Mp4Diff {
    let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(mutation, snapshot);
    *snapshot = <Mp4Diff as protocol::MutationDiff<Mp4Snapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — plain `serde_json` round-trip of the whole enum (the
/// scaffold's own honesty-boundary pattern, retained: f6-final-summary.md §4.4 documents no
/// generic collection-diff `DslField` bridge exists, and this mutation vocabulary's payload types
/// (`Mp4Snapshot`, `Mp4Track`, …) are themselves generics-heavy, so `#[derive(dsl::DslOps)]` hits
/// the same wall — hand-rolled JSON codec, not a grammar regression).
impl protocol::OpText for Mp4Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String { serde_json::to_string(self).unwrap_or_default() }
}

impl protocol::OpBinary for Mp4Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA;
    use protocol::MutationDiff;
    use protocol::command::DiffAlgebra;

    fn base_snapshot() -> Mp4Snapshot {
        Mp4Snapshot {
            schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
            ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] },
            movie: Mp4Movie::default(),
            tracks: vec![Mp4Track {
                track_id: 1,
                timescale: 1000,
                codec: Mp4Codec::Avc { sps: vec![vec![0x67]], pps: vec![vec![0x68]], nal_length_size: 4, extension: None },
                width: 64,
                height: 64,
                metadata: Mp4TrackMetadata::default(),
                chunk_sample_counts: vec![1],
                samples: vec![Mp4Sample { data: vec![1, 2, 3], duration: 33, cts_offset: 0, sync: true }],
            }],
            unknown_boxes: vec![Mp4Box { fourcc: "free".into(), data: vec![0, 0] }],
        }
    }

    /// 🧪️ mutation_diff_law + inverse_law, exercised across every real variant.
    #[test]
    fn mutation_diff_law_and_inverse_law_hold_for_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            Mp4Mutation::SetFtyp { ftyp: Mp4Ftyp { major_brand: "mp42".into(), minor_version: 1, compatible_brands: vec![] } },
            Mp4Mutation::InsertTrack { index: 1, track: Mp4Track { track_id: 2, timescale: 500, codec: Mp4Codec::Other { fourcc: "mjpg".into(), raw: vec![] }, width: 32, height: 32, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![0], samples: vec![] } },
            Mp4Mutation::SetTrackDimensions { track_index: 0, width: 128, height: 128 },
            Mp4Mutation::SetTrackCodec { track_index: 0, codec: Mp4Codec::Other { fourcc: "hvc1".into(), raw: vec![9] } },
            Mp4Mutation::InsertSample { track_index: 0, index: 1, sample: Mp4Sample { data: vec![9, 9], duration: 33, cts_offset: 0, sync: false } },
            Mp4Mutation::SetSampleSync { track_index: 0, index: 0, sync: false },
            Mp4Mutation::AddUnknownBox { index: 1, item: Mp4Box { fourcc: "uuid".into(), data: vec![1] } },
            Mp4Mutation::RemoveUnknownBox { index: 0 },
        ];
        for m in variants {
            let mut snap = base.clone();
            let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&m, &snap);
            let expected = diff.apply(&snap);
            let returned = apply_mp4_mutation(&mut snap, &m);
            assert_eq!(returned, diff, "apply_mp4_mutation must return the SAME diff as Mutation::diff for {m:?}");
            assert_eq!(snap, expected, "mutation_diff_law failed for {m:?}");

            let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&m, &base);
            assert_eq!(inv.len(), 1);
            let mut round = snap.clone();
            apply_mp4_mutation(&mut round, &inv[0]);
            assert_eq!(round, base, "inverse_law failed for {m:?}");
        }
    }

    #[test]
    fn remove_track_then_insert_track_round_trips() {
        let mut base = base_snapshot();
        base.tracks.push(Mp4Track { track_id: 2, timescale: 1000, codec: Mp4Codec::Other { fourcc: "mjpg".into(), raw: vec![] }, width: 10, height: 10, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![0], samples: vec![] });
        let m = Mp4Mutation::RemoveTrack { index: 0 };
        let mut snap = base.clone();
        let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&m, &snap);
        apply_mp4_mutation(&mut snap, &m);
        assert_eq!(snap, diff.apply(&base));
        assert_eq!(snap.tracks.len(), 1);
        let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&m, &base);
        let mut round = snap.clone();
        apply_mp4_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }

    #[test]
    fn remove_sample_then_insert_sample_round_trips() {
        let mut base = base_snapshot();
        base.tracks[0].samples.push(Mp4Sample { data: vec![4, 5], duration: 33, cts_offset: 0, sync: false });
        let m = Mp4Mutation::RemoveSample { track_index: 0, index: 0 };
        let mut snap = base.clone();
        apply_mp4_mutation(&mut snap, &m);
        assert_eq!(snap.tracks[0].samples.len(), 1);
        let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&m, &base);
        let mut round = snap.clone();
        apply_mp4_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }

    #[test]
    fn set_snapshot_still_works_as_a_full_replace() {
        let base = base_snapshot();
        let mut next = base.clone();
        next.ftyp.major_brand = "isom-mutated".into();
        let mutation = Mp4Mutation::SetSnapshot { snapshot: next.clone() };
        let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&mutation, &base);
        assert_eq!(diff.apply(&base), next);
        let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&mutation, &base);
        let mut round = next.clone();
        apply_mp4_mutation(&mut round, &inv[0]);
        assert_eq!(round, base);
    }

    /// 🧪️ op_text_binary_roundtrip_law
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for m in [
            Mp4Mutation::NoMutation,
            Mp4Mutation::SetSnapshot { snapshot: base.clone() },
            Mp4Mutation::SetFtyp { ftyp: base.ftyp.clone() },
            Mp4Mutation::RemoveTrack { index: 0 },
            Mp4Mutation::SetSampleSync { track_index: 0, index: 0, sync: true },
        ] {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = Mp4Mutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m);

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = Mp4Mutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m);
        }
    }

    #[test]
    fn exact_fixture_no_mutation_inverse_and_set_snapshot_binary_codec_preserve_source() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/bauen-mit-bestand.mp4");
        let bytes = std::fs::read(path).expect("read exact MP4 fixture");
        let base = crate::artifacts::mp4::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

        let mut unchanged = base.clone();
        apply_mp4_mutation(&mut unchanged, &Mp4Mutation::NoMutation);
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&unchanged), bytes);

        let mutation = Mp4Mutation::SetSampleSync { track_index: 0, index: 0, sync: !base.tracks[0].samples[0].sync };
        let inverse = mutation.inverse(&base);
        let mut round_trip = base.clone();
        apply_mp4_mutation(&mut round_trip, &mutation);
        apply_mp4_mutation(&mut round_trip, &inverse[0]);
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&round_trip), bytes);

        let set_snapshot = Mp4Mutation::SetSnapshot { snapshot: base };
        let encoded = set_snapshot.encode_op().expect("encode exact source set-snapshot");
        let decoded = Mp4Mutation::decode_op(&encoded).expect("decode exact source set-snapshot");
        let Mp4Mutation::SetSnapshot { snapshot } = decoded else { panic!("expected set-snapshot") };
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&snapshot), bytes);
    }
}
//#endregion 🔖️Tests
