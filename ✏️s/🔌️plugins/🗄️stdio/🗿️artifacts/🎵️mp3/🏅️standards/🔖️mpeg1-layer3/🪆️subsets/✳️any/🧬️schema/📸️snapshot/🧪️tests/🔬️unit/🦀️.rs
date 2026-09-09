use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> Mp3Snapshot {
    Mp3Snapshot {
        id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0, b's', b'x'] }] }),
        frames: vec![Mp3Frame {
            header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
            payload: vec![0u8; 413],
        }],
        ..Mp3Snapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample_snapshot();
    let bytes = <Mp3Snapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <Mp3Snapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample_snapshot();
    let text = <Mp3Snapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <Mp3Snapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}
