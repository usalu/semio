
use super::*;
use protocol::DiffCodec;

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
    let a = snapshot_a();
    let b = snapshot_b();
    let cases = vec![SemioVideoDiff::default(), SemioVideoDiff::between(&a, &b), SemioVideoDiff::between(&b, &a), SemioVideoDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioVideoDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioVideoDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }

    // Field sweep proof: confirm every collection flavor actually got exercised above.
    let diff_ab = SemioVideoDiff::between(&a, &b);
    let streams_diff = diff_ab.streams.as_ref().expect("streams diff present");
    assert!(!streams_diff.removed.is_empty(), "streams: removed not exercised");
    assert_eq!(streams_diff.modified.len(), 1);
    let stream_mod = &streams_diff.modified[0].diff;
    assert!(stream_mod.kind.is_some() && stream_mod.codec.is_some() && stream_mod.width.is_some() && stream_mod.height.is_some() && stream_mod.rate.is_some(), "modified stream: not every scalar field exercised");
    let samples_diff = stream_mod.samples.as_ref().expect("nested samples diff present");
    assert!(!samples_diff.removed.is_empty(), "samples: removed not exercised");
    assert!(!samples_diff.modified.is_empty(), "samples: modified not exercised");

    let diff_ba = SemioVideoDiff::between(&b, &a);
    let streams_diff_ba = diff_ba.streams.as_ref().expect("streams diff (b->a) present");
    assert!(!streams_diff_ba.added.is_empty(), "streams (b->a): added not exercised");
    let stream_mod_ba = &streams_diff_ba.modified[0].diff;
    let samples_diff_ba = stream_mod_ba.samples.as_ref().expect("nested samples diff (b->a) present");
    assert!(!samples_diff_ba.added.is_empty(), "samples (b->a): added not exercised");
}
