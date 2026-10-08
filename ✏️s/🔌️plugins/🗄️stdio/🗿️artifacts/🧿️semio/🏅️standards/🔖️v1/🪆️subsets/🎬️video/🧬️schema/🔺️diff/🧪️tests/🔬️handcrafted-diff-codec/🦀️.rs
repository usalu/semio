use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn snapshot_a() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "avc-old".into(),
                width: 640,
                height: 480,
                rate: SemioRational { num: 24, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![9] }, SemioVideoSample { pts: 1, key: false, data: vec![8] }, SemioVideoSample { pts: 2, key: true, data: vec![7] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn snapshot_b() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Audio,
                codec: "new-codec".into(),
                width: 1280,
                height: 720,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![9] }, SemioVideoSample { pts: 22, key: true, data: vec![80] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
        ],
    }
}

/// 🧪️ `diff_codec_text_binary_roundtrip_law`: print/parse and encode/decode round-trip over
/// the hand-rolled `SemioVideoDiff` grammar — exercises `streams.removed`/`.modified`/`.added`
/// AND, within the same modified stream, nested `samples.removed`/`.modified` (reverse
/// direction additionally exercises nested `samples.added`), a `SemioVideoStreamKind` enum
/// change, and every stream-level scalar field.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioVideoDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioVideoDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

}
