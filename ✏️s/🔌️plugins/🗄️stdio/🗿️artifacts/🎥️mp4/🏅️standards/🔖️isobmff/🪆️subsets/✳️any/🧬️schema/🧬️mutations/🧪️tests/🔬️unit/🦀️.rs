use super::*;
use crate::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA;
use protocol::MutationDiff;

async fn base_snapshot() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into()] },
        movie: Mp4Movie::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 1000,
            codec: Mp4Codec { sps: vec![vec![0x67]], pps: vec![vec![0x68]], nal_length_size: 4, extension: None },
            width: 64,
            height: 64,
            metadata: Mp4TrackMetadata::default(),
            chunk_sample_counts: vec![1],
            samples: vec![Mp4Sample { data: vec![1, 2, 3], duration: 33, cts_offset: 0, sync: true }],
        }],
    }
}

/// 🧪️ mutation_diff_law + inverse_law, exercised across every real variant.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_and_inverse_law_hold_for_every_variant() {
    let base = base_snapshot().await;
    let variants = vec![
        Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: Mp4Ftyp { major_brand: "mp42".into(), minor_version: 1, compatible_brands: vec![] } }),
        Mp4Mutation::InsertTrack(insert_track::InsertTrack {
            index: 1,
            track: Mp4Track { track_id: 2, timescale: 500, codec: Mp4Codec::default(), width: 32, height: 32, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![0], samples: vec![] },
        }),
        Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index: 0, width: 128, height: 128 }),
        Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index: 0, codec: Mp4Codec { sps: vec![vec![9]], pps: vec![vec![8]], nal_length_size: 4, extension: None } }),
        Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index: 0, index: 1, sample: Mp4Sample { data: vec![9, 9], duration: 33, cts_offset: 0, sync: false } }),
        Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: 0, index: 0, sync: false }),
    ];
    for m in variants {
        let mut snap = base.clone();
        let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&m, &snap);
        let expected = diff.diff().apply(&snap).unwrap();
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

#[semio_framework_async_macros::async_test]
async fn remove_track_then_insert_track_round_trips() {
    let mut base = base_snapshot().await;
    base.tracks.push(Mp4Track { track_id: 2, timescale: 1000, codec: Mp4Codec::default(), width: 10, height: 10, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![0], samples: vec![] });
    let m = Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: 0 });
    let mut snap = base.clone();
    let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&m, &snap);
    apply_mp4_mutation(&mut snap, &m);
    assert_eq!(snap, diff.diff().apply(&base).unwrap());
    assert_eq!(snap.tracks.len(), 1);
    let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&m, &base);
    let mut round = snap.clone();
    apply_mp4_mutation(&mut round, &inv[0]);
    assert_eq!(round, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_sample_then_insert_sample_round_trips() {
    let mut base = base_snapshot().await;
    base.tracks[0].samples.push(Mp4Sample { data: vec![4, 5], duration: 33, cts_offset: 0, sync: false });
    let m = Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index: 0, index: 0 });
    let mut snap = base.clone();
    apply_mp4_mutation(&mut snap, &m);
    assert_eq!(snap.tracks[0].samples.len(), 1);
    let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&m, &base);
    let mut round = snap.clone();
    apply_mp4_mutation(&mut round, &inv[0]);
    assert_eq!(round, base);
}

#[semio_framework_async_macros::async_test]
async fn set_snapshot_still_works_as_a_full_replace() {
    let base = base_snapshot().await;
    let mut next = base.clone();
    next.ftyp.major_brand = "isom-mutated".into();
    let mutation = Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next.clone() });
    let diff = <Mp4Mutation as Mutation<Mp4Snapshot>>::diff(&mutation, &base);
    assert_eq!(diff.diff().apply(&base).unwrap(), next);
    let inv = <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(&mutation, &base);
    let mut round = next.clone();
    apply_mp4_mutation(&mut round, &inv[0]);
    assert_eq!(round, base);
}

/// 🧪️ op_text_binary_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot().await;
    for m in [
        Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: base.ftyp.clone() }),
        Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: 0 }),
        Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: 0, index: 0, sync: true }),
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

/// 🧪️ kinds_law — `KINDS` must cover every variant, in the exact order `OpText::print_op`'s own
/// keyword derives them, so the oracle catalog's declaration is provably honest (fleet brief
/// §1: "the framework never parses Rust to check it itself").
#[semio_framework_async_macros::async_test]
async fn kinds_const_matches_enum_variants_in_declaration_order() {
    let base = base_snapshot().await;
    let one_per_variant = vec![
        Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: base.ftyp.clone() }),
        Mp4Mutation::InsertTrack(insert_track::InsertTrack { index: 1, track: base.tracks[0].clone() }),
        Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: 0 }),
        Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index: 0, width: 128, height: 128 }),
        Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index: 0, codec: base.tracks[0].codec.clone() }),
        Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index: 0, index: 0, sample: base.tracks[0].samples[0].clone() }),
        Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index: 0, index: 0 }),
        Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: 0, index: 0, sync: false }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        let printed = mutation.print_op();
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn exact_fixture_no_mutation_inverse_and_set_snapshot_binary_codec_preserve_source() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../temp/bauen-mit-bestand.mp4");
    let bytes = std::fs::read(path).expect("read exact MP4 fixture");
    let base = crate::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

    let mut unchanged = base.clone();
    let snapshot = unchanged.clone();
    apply_mp4_mutation(&mut unchanged, &Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&unchanged), bytes);

    let mutation = Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: 0, index: 0, sync: !base.tracks[0].samples[0].sync });
    let inverse = mutation.inverse(&base);
    let mut round_trip = base.clone();
    apply_mp4_mutation(&mut round_trip, &mutation);
    apply_mp4_mutation(&mut round_trip, &inverse[0]);
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&round_trip), bytes);

    let set_snapshot = Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base });
    let encoded = set_snapshot.encode_op().expect("encode exact source set-snapshot");
    let decoded = Mp4Mutation::decode_op(&encoded).expect("decode exact source set-snapshot");
    let Mp4Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) = decoded else { panic!("expected set-snapshot") };
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&snapshot), bytes);
}
