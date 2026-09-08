
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream { kind: SemioVideoStreamKind::Video, codec: "h264".into(), width: 1920, height: 1080, rate: SemioRational { num: 30, den: 1 }, samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }] },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample_snapshot();
    let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample_snapshot();
    let text = <SemioVideoSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn stream_kind_defaults_to_video_and_rational_defaults_to_one_over_one() {
    assert_eq!(SemioVideoStreamKind::default(), SemioVideoStreamKind::Video);
    assert_eq!(SemioRational::default(), SemioRational { num: 1, den: 1 });
}
