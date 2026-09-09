use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn synthetic_snapshot() -> AviSnapshot {
    AviSnapshot {
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        main_header: AviMainHeader {
            micro_sec_per_frame: 100_000,
            max_bytes_per_sec: 1400,
            padding_granularity: 0,
            flags: 0x10,
            total_frames: 2,
            initial_frames: 0,
            streams: 1,
            suggested_buffer_size: 140,
            width: 16,
            height: 16,
            reserved: vec![0, 0, 0, 0],
        },
        streams: vec![AviStream {
            strh: AviStreamHeader {
                fcc_type: "vids".into(),
                fcc_handler: "MJPG".into(),
                flags: 0,
                priority: 0,
                language: 0,
                initial_frames: 0,
                scale: 1,
                rate: 10,
                start: 0,
                length: 2,
                suggested_buffer_size: 140,
                quality: -1,
                sample_size: 0,
                rc_frame_left: 0,
                rc_frame_top: 0,
                rc_frame_right: 16,
                rc_frame_bottom: 16,
                rc_frame_width: 16,
                strh_extra: vec![],
            },
            strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
            chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, AviChunk { fourcc: "00dc".into(), data: vec![5, 6, 7], keyframe: true }],
            strl_extra: vec![],
        }],
        idx1_present: true,
        unknown_chunks: vec![],
        hdrl_extra: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn sniff_recognizes_real_riff_avi_magic() {
    let bytes = encode_avi(&synthetic_snapshot());
    assert!(sniff_real_bytes(&bytes));
    assert!(!sniff_real_bytes(b"not an avi at all!!"));
    let mut wave = b"RIFF".to_vec();
    wave.extend_from_slice(&4u32.to_le_bytes());
    wave.extend_from_slice(b"WAVE");
    assert!(!sniff_real_bytes(&wave));
}

#[semio_framework_async_macros::async_test]
async fn decode_encode_decode_round_trips_synthetic_snapshot() {
    let snap = synthetic_snapshot();
    let bytes = encode_avi(&snap);
    let back = decode_avi(&bytes).expect("decode");
    assert_eq!(back, snap);
}

#[semio_framework_async_macros::async_test]
async fn audio_stream_round_trips_via_wave_format() {
    let mut snap = synthetic_snapshot();
    snap.streams.push(AviStream {
        strh: AviStreamHeader {
            fcc_type: "auds".into(),
            fcc_handler: "    ".into(),
            flags: 0,
            priority: 0,
            language: 0,
            initial_frames: 0,
            scale: 1,
            rate: 44100,
            start: 0,
            length: 4,
            suggested_buffer_size: 4,
            quality: 0,
            sample_size: 2,
            rc_frame_left: 0,
            rc_frame_top: 0,
            rc_frame_right: 0,
            rc_frame_bottom: 0,
            rc_frame_width: 16,
            strh_extra: vec![],
        },
        strf: AviStreamFormat::WaveFormat { format_tag: 1, channels: 1, samples_per_sec: 44100, avg_bytes_per_sec: 88200, block_align: 2, bits_per_sample: 16, extra: vec![] },
        chunks: vec![AviChunk { fourcc: "01wb".into(), data: vec![9, 9], keyframe: true }],
        strl_extra: vec![],
    });
    snap.main_header.streams = 2;
    let bytes = encode_avi(&snap);
    let back = decode_avi(&bytes).expect("decode");
    assert_eq!(back, snap);
}

#[semio_framework_async_macros::async_test]
async fn no_idx1_still_round_trips() {
    let mut snap = synthetic_snapshot();
    snap.idx1_present = false;
    let bytes = encode_avi(&snap);
    let back = decode_avi(&bytes).expect("decode");
    assert_eq!(back, snap);
}

//#region codec_retention_law — the REAL W0 fixture
/// 🎬️ The handcrafted-but-real `🎬️.avi` fixture (`generators/w0-fixtures/make_avi.py`,
/// see `fixtures/avi/NOTES.md`): 732 bytes, 16×16 MJPG, 3 `00dc` frames, real `idx1`.
const REAL_EXAMPLE_AVI: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎬️.avi");

#[semio_framework_async_macros::async_test]
async fn codec_retention_law_decodes_the_real_fixture_with_expected_shape() {
    let snap = decode_avi(REAL_EXAMPLE_AVI).expect("decode the real fixture");
    assert_eq!(snap.main_header.width, 16);
    assert_eq!(snap.main_header.height, 16);
    assert_eq!(snap.main_header.total_frames, 3);
    assert!(snap.idx1_present);
    assert_eq!(snap.streams.len(), 1);
    let stream = &snap.streams[0];
    assert_eq!(stream.strh.fcc_type, "vids");
    assert_eq!(stream.strh.fcc_handler, "MJPG");
    assert_eq!(stream.chunks.len(), 3, "NOTES.md: 3 00dc frame chunks");
    assert!(stream.chunks.iter().all(|c| c.keyframe), "NOTES.md: idx1 AVIIF_KEYFRAME set on all 3 entries");
    match &stream.strf {
        AviStreamFormat::BitmapInfo { width, height, compression, .. } => {
            assert_eq!(*width, 16);
            assert_eq!(*height, 16);
            assert_eq!(compression, "MJPG");
        }
        other => panic!("expected BitmapInfo, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_law_round_trips_the_real_fixture_byte_identically() {
    // 🧪️ This fixture is simple enough (single stream, no untyped `hdrl` auxiliary fields
    // beyond what `AviMainHeader`/`AviStreamHeader`/`AviStreamFormat` fully type) that this
    // engine achieves LITERAL byte-for-byte round-tripping, not just documented-normal-form —
    // the strongest form of codec_retention_law.
    let snap = decode_avi(REAL_EXAMPLE_AVI).expect("decode");
    let re_encoded = encode_avi(&snap);
    assert_eq!(re_encoded, REAL_EXAMPLE_AVI, "decode(bytes) -> encode(..) must reproduce the real fixture byte-for-byte");

    let round_tripped = decode_avi(&re_encoded).expect("re-decode");
    assert_eq!(round_tripped, snap);

    for chunk in &snap.streams[0].chunks {
        assert!(REAL_EXAMPLE_AVI.windows(chunk.data.len().max(1)).any(|w| w == chunk.data.as_slice()), "chunk data must be a verbatim slice of the real source file");
    }
}
//#endregion codec_retention_law

//#region real_ffmpeg_fixture — BUG 1 + BUG 2, see the ticket's own w7-avi-1-0-mutate-report.md
/// 🎥️ Real 3-second Motion-JPEG AVI-1.0, ffmpeg-derived from this repository's only real video
/// (`♻️mit-bestand/.../🎥️bauen-mit-bestand.mp4`). Its own `strh` is 56 bytes (BUG 1: ffmpeg's
/// AVI-1.0 muxer writes the classic form) and its `hdrl`/`strl` carry real `JUNK`/`vprp`
/// auxiliary chunks (BUG 2) — confirmed by direct hex inspection, not assumed.
const REAL_FFMPEG_AVI: &[u8] = include_bytes!("../../../🧫️fixtures/🎬️.avi");

#[semio_framework_async_macros::async_test]
async fn decode_avi_accepts_the_real_ffmpeg_56_byte_strh() {
    let snap = decode_avi(REAL_FFMPEG_AVI).expect("decode_avi must accept a real ffmpeg AVI-1.0 strh (56 bytes, classic SHORT-rcFrame form)");
    assert_eq!(snap.streams.len(), 1);
    let strh = &snap.streams[0].strh;
    assert_eq!(strh.fcc_type, "vids");
    assert_eq!(strh.fcc_handler, "MJPG");
    assert_eq!(strh.rc_frame_width, 8, "the real fixture's strh is the classic 56-byte SHORT-rcFrame form, not the 64-byte LONG form");
    assert_eq!((strh.rc_frame_left, strh.rc_frame_top, strh.rc_frame_right, strh.rc_frame_bottom), (0, 0, 480, 432), "rcFrame read as 4 SHORTs must be the real 480x432 frame rectangle, not misread as 2 LONGs");
}

#[semio_framework_async_macros::async_test]
async fn decode_avi_retains_nested_hdrl_and_strl_auxiliary_chunks() {
    let snap = decode_avi(REAL_FFMPEG_AVI).expect("decode");
    assert!(snap.hdrl_extra.iter().any(|c| c.fourcc == "JUNK"), "the real fixture's hdrl carries its own 260-byte JUNK padding chunk");
    let strl_extra = &snap.streams[0].strl_extra;
    assert!(strl_extra.iter().any(|c| c.fourcc == "JUNK"), "the real fixture's strl carries a 4120-byte JUNK padding chunk");
    assert!(strl_extra.iter().any(|c| c.fourcc == "vprp"), "the real fixture's strl carries a real vprp (video properties) chunk");
}

#[semio_framework_async_macros::async_test]
async fn decode_encode_round_trips_the_real_ffmpeg_fixtures_strh_and_nested_chunks() {
    let snap = decode_avi(REAL_FFMPEG_AVI).expect("decode");
    let re_encoded = encode_avi(&snap);
    let round_tripped = decode_avi(&re_encoded).expect("re-decode");
    assert_eq!(round_tripped, snap, "decode -> encode -> decode must be a fixed point, including the 56-byte strh width and the nested hdrl/strl auxiliaries");
}
//#endregion real_ffmpeg_fixture
