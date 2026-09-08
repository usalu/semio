
use super::*;
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;

async fn synthetic_snapshot() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "avc1".into(), "mp41".into()] },
        movie: Mp4Movie::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 90000,
            codec: Mp4Codec { sps: vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], pps: vec![vec![0x68, 0xCE, 0x3C, 0x80]], nal_length_size: 4, extension: None },
            width: 64,
            height: 64,
            metadata: Mp4TrackMetadata::default(),
            chunk_sample_counts: vec![3],
            samples: vec![
                Mp4Sample { data: vec![0, 0, 0, 6, 0x65, 1, 2, 3, 4, 5], duration: 3000, cts_offset: 0, sync: true },
                Mp4Sample { data: vec![0, 0, 0, 4, 0x61, 6, 7, 8], duration: 3000, cts_offset: 3000, sync: false },
                Mp4Sample { data: vec![0, 0, 0, 4, 0x61, 9, 10, 11], duration: 3000, cts_offset: 0, sync: false },
            ],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn sniff_recognizes_real_ftyp_magic_only() {
    let bytes = encode_mp4(&synthetic_snapshot().await);
    assert!(sniff_real_bytes(&bytes));
    assert!(!sniff_real_bytes(b"not an mp4 at all"));
    assert!(!sniff_real_bytes(&[0u8, 0, 0, 8, b'f', b'r', b'e', b'e']));
}

#[semio_framework_async_macros::async_test]
async fn decode_encode_decode_round_trips_synthetic_snapshot() {
    let snap = synthetic_snapshot().await;
    let bytes = encode_mp4(&snap);
    let back = decode_mp4(&bytes).expect("decode");
    assert_eq!(back, snap, "decode(encode(snapshot)) must reproduce the snapshot exactly");
}

//#region codec_retention_law — the REAL 43KB fixture
/// 🎬️ The real 43KB `logo.mp4` (copied verbatim from `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🎞️animation/🎬️animation.mp4`
/// into this artifact's own examples per W0/W1b — see `fixtures/mp4/NOTES.md` in the ticket
/// folder: `ffprobe` confirms `codec_name=h264, width=410, height=140, nb_frames=1441,
/// nal_length_size=4, extradata_size=46`).
const REAL_LOGO_MP4: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎬️.mp4");

#[semio_framework_async_macros::async_test]
async fn codec_retention_law_decodes_the_real_fixture_with_expected_shape() {
    let snap = decode_mp4(REAL_LOGO_MP4).expect("decode the real 43KB fixture");
    assert_eq!(snap.ftyp.major_brand, "isom");
    assert!(snap.ftyp.compatible_brands.iter().any(|b| b == "avc1"), "compatible_brands: {:?}", snap.ftyp.compatible_brands);
    assert_eq!(snap.tracks.len(), 1, "logo.mp4 has exactly one (video) track");
    let track = &snap.tracks[0];
    assert_eq!(track.width, 410);
    assert_eq!(track.height, 140);
    assert_eq!(track.samples.len(), 1441, "ffprobe nb_frames=1441");
    assert_eq!(track.codec.nal_length_size, 4, "ffprobe nal_length_size=4");
    assert!(!track.codec.sps.is_empty() && !track.codec.pps.is_empty(), "avcC must carry real SPS/PPS (extradata_size=46)");
    assert!(track.samples[0].sync, "the first sample of a real mp4 is always a sync/IDR sample");
    assert!(track.samples.iter().any(|s| !s.data.is_empty()), "sample payload bytes must be real, not fabricated");
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_law_round_trips_the_real_fixture_snapshot_exactly() {
    // 🧪️ Strongest provable claim within this codec's documented normal-form scope (see this
    // module's doc comment): decode -> encode -> re-decode reproduces the EXACT same
    // snapshot — every sample's bytes/duration/cts_offset/sync flag, every track field, ftyp,
    // and every named logical field survives through a real mux/demux cycle on
    // real, non-synthetic, 1441-frame H.264 data.
    let snap = decode_mp4(REAL_LOGO_MP4).expect("decode");
    let re_encoded = encode_mp4(&snap);
    let round_tripped = decode_mp4(&re_encoded).expect("re-decode the round-tripped bytes");
    assert_eq!(round_tripped, snap, "decode(encode(decode(real_fixture))) must equal decode(real_fixture)");

    // 🧪️ Sample PAYLOAD bytes (the actual codec substance) are byte-exact against the ORIGINAL
    // file bytes too, not just self-consistent with our own re-encode — every sample's `data`
    // must appear verbatim somewhere in the source file (proof the bytes were genuinely read
    // from `mdat`, never fabricated).
    for sample in &snap.tracks[0].samples[..50.min(snap.tracks[0].samples.len())] {
        assert!(REAL_LOGO_MP4.windows(sample.data.len().max(1)).any(|w| w == sample.data.as_slice()), "sample data must be a verbatim slice of the real source file");
    }
}

#[semio_framework_async_macros::async_test]
async fn exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte() {
    use crate::standards::isobmff::subsets::any::schema::{
        Mp4AnalyzerAnalysis,
        diff::Mp4Diff,
        mutations::{Mp4Mutation, apply_mp4_mutation},
    };
    use protocol::{DiffCodec, Mutation, OpBinary, OpText};
    use semio_framework_plugin::{AnalyzeSource, ArtifactAnalysis, ArtifactComposition, ComposeSource};

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../temp/bauen-mit-bestand.mp4");
    let bytes = std::fs::read(path).expect("read exact MP4 fixture");
    let snapshot = decode_mp4(&bytes).expect("decode exact MP4 fixture");
    assert_eq!(encode_mp4(&snapshot), bytes);

    let pack = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let from_pack = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode MP4 pack");
    assert_eq!(encode_mp4(&from_pack), bytes);

    let dsl = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let from_dsl = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse MP4 DSL");
    assert_eq!(encode_mp4(&from_dsl), bytes);

    let analysis = Mp4AnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&pack)]);
    let analyzed = analysis.parts.snapshot.expect("MP4 analyzer snapshot");
    assert_eq!(encode_mp4(&analyzed), bytes);

    let dialect = <Mp4AnalyzerAnalysis as ArtifactAnalysis>::DIALECT;
    let composition = Mp4ComposerComposition::compose(&[ComposeSource { dialect, payload: AnalyzeSource::Binary(&pack) }]).expect("compose MP4 pack");
    assert_eq!(encode_mp4(&composition.snapshot), bytes);

    let self_diff = Mp4Diff::between(&snapshot, &snapshot);
    let text_diff = Mp4Diff::parse_diff(&self_diff.print_diff()).expect("parse MP4 diff text");
    assert_eq!(encode_mp4(&text_diff.apply(&snapshot).unwrap()), bytes);
    let binary_diff = Mp4Diff::decode_diff(&self_diff.encode_diff().expect("encode MP4 diff")).expect("decode MP4 diff");
    assert_eq!(encode_mp4(&binary_diff.apply(&snapshot).unwrap()), bytes);

    let mut no_op = snapshot.clone();
    let no_op_mutation = Mp4Mutation::SetSnapshot(crate::standards::isobmff::subsets::any::schema::mutations::set_snapshot::SetSnapshot { snapshot: no_op.clone() });
    assert!(apply_mp4_mutation(&mut no_op, &no_op_mutation).diff().is_empty());
    assert_eq!(encode_mp4(&no_op), bytes);

    let set_snapshot = Mp4Mutation::SetSnapshot(crate::standards::isobmff::subsets::any::schema::mutations::set_snapshot::SetSnapshot { snapshot: snapshot.clone() });
    let text_op = Mp4Mutation::parse_op(&set_snapshot.print_op()).expect("parse MP4 operation text");
    let mut from_text_op = Mp4Snapshot::default();
    apply_mp4_mutation(&mut from_text_op, &text_op);
    assert_eq!(encode_mp4(&from_text_op), bytes);
    let binary_op = Mp4Mutation::decode_op(&set_snapshot.encode_op().expect("encode MP4 operation")).expect("decode MP4 operation");
    let mut from_binary_op = Mp4Snapshot::default();
    apply_mp4_mutation(&mut from_binary_op, &binary_op);
    assert_eq!(encode_mp4(&from_binary_op), bytes);

    let mut changed = snapshot.clone();
    let mutation = Mp4Mutation::SetTrackDimensions(crate::standards::isobmff::subsets::any::schema::mutations::set_track_dimensions::SetTrackDimensions { track_index: 0, width: snapshot.tracks[0].width + 1, height: snapshot.tracks[0].height });
    apply_mp4_mutation(&mut changed, &mutation);
    let changed_bytes = encode_mp4(&changed);
    assert_ne!(changed_bytes, bytes, "semantic mutation must materialize changed logical state");

    let diff = Mp4Diff::between(&snapshot, &changed);
    let after = diff.apply(&snapshot).unwrap();
    let restored = diff.inverse(&snapshot).apply(&after).unwrap();
    assert_eq!(restored, snapshot, "mutation inverse must reconstruct the logical snapshot");
    assert_eq!(encode_mp4(&restored), bytes, "restored logical state must materialize the imported MP4 exactly");
    for inverse in mutation.inverse(&snapshot) {
        apply_mp4_mutation(&mut changed, &inverse);
    }
    assert_eq!(encode_mp4(&changed), bytes);
}
//#endregion codec_retention_law

//#region chunk_grouping_reconciliation
/// 🧮️ `chunk_sample_counts` is a RETAINED `stsc`/`stco` layout hint, and the sample list is the
/// truth. When the two disagree the encoder normalizes to one chunk per track instead of
/// aborting — reachable by construction, not a defensive nicety: `SetSnapshot` carries a whole
/// caller-supplied document and a caller that drops a sample cannot be expected to restate a
/// writer's chunking. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR: `mutate-mp4-isobmff`'s
/// `set-snapshot` row drops the first track's last sample, and this encoder used to panic on it
/// (`MP4 chunk sample counts must cover every sample`, left 47 right 46), taking the whole
/// subject host down before a single scenario could report.
#[test]
fn a_stale_chunk_grouping_is_reconciled_against_the_sample_list() {
    let sample = |n: u8| Mp4Sample { data: vec![0, 0, 0, 1, n], duration: 1000, cts_offset: 0, sync: true };
    let track = |counts: Vec<u32>, samples: Vec<Mp4Sample>| Mp4Track {
        track_id: 1,
        timescale: 1000,
        codec: Mp4Codec { sps: vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], pps: vec![vec![0x68, 0xCE, 0x3C, 0x80]], nal_length_size: 4, extension: None },
        width: 16,
        height: 16,
        metadata: Mp4TrackMetadata::default(),
        chunk_sample_counts: counts,
        samples,
    };
    assert_eq!(normalized_chunk_sample_counts(&track(vec![2, 1], vec![sample(1), sample(2), sample(3)])), vec![2, 1], "a grouping that partitions the sample list is retained verbatim");
    assert_eq!(normalized_chunk_sample_counts(&track(vec![2, 1], vec![sample(1), sample(2)])), vec![2], "a grouping left over from a longer sample list normalizes to one chunk");
    assert_eq!(normalized_chunk_sample_counts(&track(vec![1], vec![sample(1), sample(2)])), vec![2], "a grouping left over from a shorter sample list normalizes to one chunk");
    assert_eq!(normalized_chunk_sample_counts(&track(vec![], vec![sample(1)])), vec![1], "no retained grouping at all is the same one-chunk normal form");

    let stale = Mp4Snapshot {
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "avc1".into()] },
        movie: Mp4Movie::default(),
        tracks: vec![track(vec![3], vec![sample(1), sample(2)])],
    };
    let bytes = encode_mp4(&stale);
    assert!(sniff_real_bytes(&bytes), "a snapshot whose retained grouping is stale still encodes to a real MP4");
}
//#endregion chunk_grouping_reconciliation
