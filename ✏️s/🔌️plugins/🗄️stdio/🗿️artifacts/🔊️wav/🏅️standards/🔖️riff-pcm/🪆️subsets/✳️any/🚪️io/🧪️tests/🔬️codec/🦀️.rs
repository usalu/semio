use super::*;

/// 🌱 Real ~1s 440Hz mono 8kHz 16-bit PCM fixture — byte-identical to the artifact's own
/// `📚️examples/🎬️demo/🖼️assets/🔊️example.wav` (per ticket `fixtures/wav/NOTES.md`), duplicated
/// here as a literal so the test doesn't reach across an emoji-path `include_bytes!` boundary.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_fixture() -> Vec<u8> {
    include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav").to_vec()
}

#[semio_framework_async_macros::async_test]
async fn sniffs_and_decodes_a_synthetic_fmt_chunk() {
    let snap = WavSnapshot { fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 8000, byte_rate: 16000, block_align: 2, bits_per_sample: 16, ext: None }, data: WavData::Pcm16(vec![0, 100, -100]), ..WavSnapshot::default() };
    let bytes = encode_wav(&snap);
    assert!(sniff_real_bytes(&bytes));
    let decoded = decode_wav(&bytes).expect("decode");
    assert_eq!(decoded.fmt, snap.fmt);
    assert_eq!(decoded.data, snap.data);
}

#[semio_framework_async_macros::async_test]
async fn sniff_rejects_non_wave_riff() {
    let mut bytes = b"RIFF".to_vec();
    bytes.extend_from_slice(&4u32.to_le_bytes());
    bytes.extend_from_slice(b"AVI ");
    assert!(!sniff_real_bytes(&bytes));
}

//#region codec_retention_law
/// 🧪️ `codec_retention_law`: decoding the REAL fixture, re-encoding, and decoding again must
/// be byte-exact at every level — the on-disk bytes, the recovered PCM samples, AND (an
/// independent confirmation, not reusing the decoder's own sample array) a freshly
/// re-synthesized 440Hz reference tone, per `fixtures/wav/NOTES.md`'s own verification
/// method.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let fixture = real_fixture();
    let decoded = decode_wav(&fixture).expect("decode real fixture");
    assert_eq!(decoded.fmt.audio_format, 1, "PCM");
    assert_eq!(decoded.fmt.channels, 1, "mono");
    assert_eq!(decoded.fmt.sample_rate, 8000);
    assert_eq!(decoded.fmt.byte_rate, 16000);
    assert_eq!(decoded.fmt.block_align, 2);
    assert_eq!(decoded.fmt.bits_per_sample, 16);
    assert_eq!(decoded.fmt.ext, None);
    assert!(decoded.other_chunks.is_empty(), "fixture has only fmt + data");

    let samples = match &decoded.data {
        WavData::Pcm16(s) => s.clone(),
        other => panic!("expected Pcm16, got {other:?}"),
    };
    assert_eq!(samples.len(), 8000, "1.0s at 8000Hz");

    // 🔬️ Independent re-synthesis (not reusing the writer's array): max abs diff must be 0.
    // `make_wav.py`'s own `max_amp = int(AMPLITUDE * 32767)` truncates to an integer BEFORE
    // multiplying by `sin(t)` — reproduced here bit-for-bit (not `0.5 * 32767.0` as a
    // continuous `f64`, which rounds a peak sample to 16384 instead of the fixture's 16383).
    let max_amp = (0.5 * 32767.0) as i32 as f64;
    let mut max_abs_diff = 0i32;
    for (n, &sample) in samples.iter().enumerate() {
        let reference = ((2.0 * std::f64::consts::PI * 440.0 * n as f64 / 8000.0).sin() * max_amp).round() as i32;
        max_abs_diff = max_abs_diff.max((sample as i32 - reference).abs());
    }
    assert_eq!(max_abs_diff, 0, "decoded samples must exactly match a freshly re-synthesized 440Hz sine");

    // 🔁️ Re-encode must reproduce the real fixture byte-for-byte (no other_chunks, no ext).
    let re_encoded = encode_wav(&decoded);
    assert_eq!(re_encoded, fixture, "encode(decode(real fixture)) must be byte-identical");

    // 🔁️ Second round trip (decode the re-encoded bytes) must also match exactly.
    let re_decoded = decode_wav(&re_encoded).expect("decode re-encoded");
    assert_eq!(re_decoded, decoded);
}
//#endregion codec_retention_law

//#region 🔖️OtherChunksRetention
#[semio_framework_async_macros::async_test]
async fn other_chunks_round_trip_verbatim_in_order() {
    let snap = WavSnapshot {
        fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 44100, byte_rate: 88200, block_align: 2, bits_per_sample: 16, ext: None },
        data: WavData::Pcm16(vec![1, 2, 3]),
        other_chunks: vec![
            RiffChunk { fourcc: "fact".into(), data: vec![0x03, 0x00, 0x00, 0x00] },
            RiffChunk { fourcc: "LIST".into(), data: b"INFOICRDodd".to_vec() }, // 🧮️ odd length exercises pad-byte handling
        ],
        ..WavSnapshot::default()
    };
    let bytes = encode_wav(&snap);
    let decoded = decode_wav(&bytes).expect("decode");
    assert_eq!(decoded.other_chunks, snap.other_chunks);
    assert_eq!(decoded.other_chunks[0].fourcc, "fact");
    assert_eq!(decoded.other_chunks[1].fourcc, "LIST");
}
//#endregion 🔖️OtherChunksRetention

//#region 🔖️ExtFmtRetention
#[semio_framework_async_macros::async_test]
async fn extensible_fmt_chunk_round_trips_ext_bytes() {
    let snap = WavSnapshot {
        fmt: WavFmt {
            audio_format: 0xFFFE,
            channels: 2,
            sample_rate: 48000,
            byte_rate: 192000,
            block_align: 4,
            bits_per_sample: 16,
            ext: Some(vec![0x16, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71]),
        },
        data: WavData::Raw(vec![0xAA, 0xBB]),
        ..WavSnapshot::default()
    };
    let bytes = encode_wav(&snap);
    let decoded = decode_wav(&bytes).expect("decode");
    assert_eq!(decoded.fmt, snap.fmt);
    assert_eq!(decoded.data, snap.data);
}
//#endregion 🔖️ExtFmtRetention
