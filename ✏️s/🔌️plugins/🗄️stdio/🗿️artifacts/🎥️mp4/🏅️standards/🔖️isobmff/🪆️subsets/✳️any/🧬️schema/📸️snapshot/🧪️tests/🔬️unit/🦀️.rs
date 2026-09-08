
use super::*;

async fn sample_snapshot() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "iso2".into(), "avc1".into(), "mp41".into()] },
        movie: Mp4Movie::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 1000,
            codec: Mp4Codec { sps: vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], pps: vec![vec![0x68, 0xCE, 0x3C, 0x80]], nal_length_size: 4, extension: None },
            width: 64,
            height: 64,
            metadata: Mp4TrackMetadata::default(),
            chunk_sample_counts: vec![2],
            samples: vec![Mp4Sample { data: vec![0, 0, 0, 4, 0x65, 1, 2, 3], duration: 33, cts_offset: 0, sync: true }, Mp4Sample { data: vec![0, 0, 0, 3, 0x61, 4, 5], duration: 33, cts_offset: 33, sync: false }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips_via_real_mp4_bytes() {
    let snap = sample_snapshot().await;
    let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_via_real_mp4_bytes() {
    let snap = sample_snapshot().await;
    let text = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips_through_real_codec() {
    let snap = Mp4Snapshot::default();
    let bytes = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let model = format!("{:?}", sample_snapshot().await);
    for forbidden in ["unknownBoxes", "physical", "sourceBytes", "nativeArchive", "\"raw\""] {
        assert!(!model.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        include_str!("../../🟦️.ts"),
        include_str!("../../🔗️.graphql"),
        include_str!("../../🔣️.json"),
        include_str!("../../🛰️.proto"),
        include_str!("../../../🔺️diff/🦀️.rs"),
        include_str!("../../../🔺️diff/🟦️.ts"),
        include_str!("../../../🔺️diff/🔗️.graphql"),
        include_str!("../../../🔺️diff/🔣️.json"),
        include_str!("../../../🔺️diff/🛰️.proto"),
        include_str!("../../../🧬️mutations/🦀️.rs"),
        include_str!("../../../🧬️mutations/🟦️.ts"),
        include_str!("../../../🧬️mutations/🔗️.graphql"),
        include_str!("../../../🧬️mutations/🔣️.json"),
        include_str!("../../../🧬️mutations/🛰️.proto"),
        include_str!("../../💾️binary/🔠️.abnf"),
        include_str!("../../💾️binary/📡️.protocol.semio"),
        include_str!("../../../🧬️mutations/📝️text/📖️.grammar.semio"),
        include_str!("../../../🧬️mutations/📝️text/🔤️.ebnf"),
        include_str!("../../../🧬️mutations/📝️text/🅰️.g4"),
        include_str!("../../../🧬️mutations/💾️binary/📡️.protocol.semio"),
        include_str!("../../../🧬️mutations/💾️binary/🌶️.spicy"),
    ] {
        for forbidden in ["unknownBoxes", "unknown_boxes", "Mp4CodecOther", "ADD_UNKNOWN_BOX", "addUnknownBox", "nativeArchive", "sourceBytes", "serde_json::", "json_line", "jsonLine", "json_utf8", "JSON bytes", "iso-bmff-box-stream"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn exact_fixture_survives_pack_and_dsl_codecs() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../temp/bauen-mit-bestand.mp4");
    let bytes = std::fs::read(path).expect("read exact MP4 fixture");
    let snapshot = crate::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

    let pack = <Mp4Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let from_pack = <Mp4Snapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode pack");
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&from_pack), bytes);

    let dsl = <Mp4Snapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let from_dsl = <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse dsl");
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&from_dsl), bytes);
}
