use super::*;

/// 🌱 Real ~1s 440Hz mono 8kHz 16-bit PCM fixture — byte-identical to the artifact's own
/// `📚️examples/🎬️demo/🖼️assets/🔊️example.wav` (per ticket `fixtures/wav/NOTES.md`), duplicated
/// here as a literal so the test doesn't reach across an emoji-path `include_bytes!` boundary.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_fixture() -> Vec<u8> {
    include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav").to_vec()
}

fn serialization_boundary(name: &str) -> usize {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧭️serialization-boundaries/🔣️.json")).expect("serialization boundary fixture is JSON");
    fixture[name].as_u64().unwrap_or_else(|| panic!("serialization boundary {name:?} is an unsigned integer")) as usize
}

fn serialization_boundary_text(name: &str) -> String {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧭️serialization-boundaries/🔣️.json")).expect("serialization boundary fixture is JSON");
    fixture[name].as_str().unwrap_or_else(|| panic!("serialization boundary {name:?} is text")).to_string()
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
            RiffChunk { fourcc: "fact".into(), data: vec![0x03, 0x00, 0x00, 0x00], pad_byte: 0 },
            RiffChunk { fourcc: "LIST".into(), data: b"INFOICRDodd".to_vec(), pad_byte: 0xA5 },
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

#[semio_framework_async_macros::async_test]
async fn complete_chunk_sequence_preserves_order_and_duplicate_canonical_chunks() {
    use std::io::Cursor;
    let snapshot: WavSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../🧫️fixtures/🧭️preserve-chunk-sequence/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral ordered-chunk fixture decodes");
    let encoded = encode_wav(&snapshot);
    let mut cursor = Cursor::new(&encoded);
    let riff = riff::Chunk::read(&mut cursor, 0).expect("riff oracle reads WAVE container");
    assert_eq!(riff.id().as_str(), "RIFF");
    assert_eq!(riff.read_type(&mut cursor).expect("riff oracle reads WAVE form").as_str(), "WAVE");
    let chunks = riff.iter(&mut cursor).collect::<Result<Vec<_>, _>>().expect("riff oracle walks children");
    let ids = chunks.iter().map(|chunk| chunk.id().as_str().to_string()).collect::<Vec<_>>();
    assert_eq!(ids, ["JUNK", "fmt ", "fmt ", "LIST", "data", "data"]);
    let payloads = chunks.iter().map(|chunk| chunk.read_contents(&mut cursor).expect("riff oracle reads child payload")).collect::<Vec<_>>();
    assert_eq!(payloads[0], [1, 2, 3]);
    assert_eq!(payloads[1], [1, 0, 1, 0, 64, 31, 0, 0, 128, 62, 0, 0, 2, 0, 16, 0]);
    assert_eq!(payloads[2], payloads[1]);
    assert_eq!(payloads[3], b"INFO");
    assert_eq!(payloads[4], [1, 0, 255, 255]);
    assert_eq!(payloads[5], [9, 0, 10, 0]);
    let junk = encoded.windows(4).position(|window| window == b"JUNK").expect("JUNK chunk encoded");
    assert_eq!(encoded[junk + 8..junk + 12], [1, 2, 3, 0xA5]);
    let decoded = decode_wav(&encoded).expect("subject decodes ordered duplicate chunks");
    assert_eq!(decoded, snapshot);
    assert_eq!(encode_wav(&decoded), encoded);

    let retained_duplicates = snapshot.other_chunks.clone();
    let mut edited = snapshot;
    edited.fmt.sample_rate = 16_000;
    edited.fmt.byte_rate = 32_000;
    edited.data = WavData::Pcm16(vec![123, -456]);
    let edited_bytes = encode_wav(&edited);
    let mut edited_cursor = Cursor::new(&edited_bytes);
    let edited_riff = riff::Chunk::read(&mut edited_cursor, 0).expect("riff oracle reads edited WAVE container");
    let edited_chunks = edited_riff.iter(&mut edited_cursor).collect::<Result<Vec<_>, _>>().expect("riff oracle walks edited children");
    let edited_payloads = edited_chunks.iter().map(|chunk| chunk.read_contents(&mut edited_cursor).expect("riff oracle reads edited payload")).collect::<Vec<_>>();
    assert_eq!(&edited_payloads[1][4..12], &[128, 62, 0, 0, 0, 125, 0, 0]);
    assert_eq!(edited_payloads[2], payloads[2]);
    assert_eq!(edited_payloads[4], [123, 0, 56, 254]);
    assert_eq!(edited_payloads[5], payloads[5]);
    let reopened = decode_wav(&edited_bytes).expect("edited primary chunks reopen");
    assert_eq!(reopened.fmt.sample_rate, 16_000);
    assert_eq!(reopened.fmt.byte_rate, 32_000);
    assert_eq!(reopened.data, WavData::Pcm16(vec![123, -456]));
    assert_eq!(reopened.other_chunks, retained_duplicates);
    assert_eq!(reopened.chunk_order, edited.chunk_order);

    edited.chunk_order = vec![
        WavChunkRef::Other(0),
        WavChunkRef::Other(1),
        WavChunkRef::Format,
        WavChunkRef::Other(2),
        WavChunkRef::Other(3),
        WavChunkRef::Samples,
    ];
    let protected = decode_wav(&encode_wav(&edited)).expect("misordered duplicate cannot mask edited primary");
    assert_eq!(protected.fmt.sample_rate, 16_000);
    assert_eq!(protected.data, WavData::Pcm16(vec![123, -456]));
    assert_eq!(protected.other_chunks, retained_duplicates);
    assert_eq!(protected.chunk_order[1..3], [WavChunkRef::Format, WavChunkRef::Other(1)]);
    assert_eq!(protected.chunk_order[4..6], [WavChunkRef::Samples, WavChunkRef::Other(3)]);
}

#[semio_framework_async_macros::async_test]
async fn exact_serialization_boundaries_survive_typed_edit_save_and_reopen() {
    use crate::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation,WavMutation};

    use std::io::Cursor;

    let maximum_ext = serialization_boundary("maximumFmtExtensionBytes");
    let mut expected = WavSnapshot {
        fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 8_000, byte_rate: 8_000, block_align: 1, bits_per_sample: 8, ext: Some(vec![0x2a; maximum_ext]) },
        data: WavData::Pcm8(vec![1, 2, 3]),
        fmt_pad_byte: serialization_boundary("validOddFmtPadByte") as u8,
        data_pad_byte: serialization_boundary("validOddDataPadByte") as u8,
        other_chunks: vec![RiffChunk {
            fourcc: serialization_boundary_text("validFourcc"),
            data: vec![4, 5, 6],
            pad_byte: serialization_boundary("validOddOtherPadByte") as u8,
        }],
        chunk_order: vec![WavChunkRef::Format, WavChunkRef::Samples, WavChunkRef::Other(0)],
        ..WavSnapshot::default()
    };
    let mut edited = protocol::apply_diff(&WavDiff::between(&WavSnapshot::default(), &expected), &WavSnapshot::default()).expect("the exact boundary state is a representable diff");
    assert_eq!(edited, expected);

    let encoded = try_encode_wav(&edited).expect("exact boundary snapshot encodes");
    let mut cursor = Cursor::new(&encoded);
    let riff = riff::Chunk::read(&mut cursor, 0).expect("independent RIFF oracle reads boundary WAVE");
    let chunks = riff.iter(&mut cursor).collect::<Result<Vec<_>, _>>().expect("independent RIFF oracle walks boundary chunks");
    assert_eq!(chunks.iter().map(|chunk| chunk.id().as_str().to_string()).collect::<Vec<_>>(), ["fmt ", "data", "JUNK"]);
    assert_eq!(chunks[0].len(), u32::try_from(18 + maximum_ext).expect("fmt boundary fits RIFF u32"));
    let fmt_payload = chunks[0].read_contents(&mut cursor).expect("independent RIFF oracle reads fmt payload");
    assert_eq!(&fmt_payload[16..18], &u16::MAX.to_le_bytes());
    assert_eq!(fmt_payload[18..], vec![0x2a; maximum_ext]);
    assert_eq!(chunks[1].read_contents(&mut cursor).expect("independent RIFF oracle reads data payload"), [1, 2, 3]);
    assert_eq!(chunks[2].read_contents(&mut cursor).expect("independent RIFF oracle reads auxiliary payload"), [4, 5, 6]);

    let fmt_offset = encoded.windows(4).position(|window| window == b"fmt ").expect("fmt chunk exists");
    let data_offset = encoded.windows(4).position(|window| window == b"data").expect("data chunk exists");
    let other_offset = encoded.windows(4).position(|window| window == b"JUNK").expect("auxiliary chunk exists");
    assert_eq!(encoded[fmt_offset + 8 + 18 + maximum_ext], expected.fmt_pad_byte);
    assert_eq!(encoded[data_offset + 8 + 3], expected.data_pad_byte);
    assert_eq!(encoded[other_offset + 8 + 3], expected.other_chunks[0].pad_byte);
    let reopened = decode_wav(&encoded).expect("boundary WAV reopens");
    assert_eq!(reopened, expected);
    assert_eq!(try_encode_wav(&reopened).expect("reopened boundary WAV saves"), encoded);

    expected.fmt.sample_rate = 16_000;
    expected.fmt.byte_rate = 16_000;
    let outcome = apply_wav_mutation(&mut edited, &WavMutation::SetFmt(set_fmt::SetFmt { fmt: expected.fmt.clone() }));
    assert!(outcome.messages().is_empty(), "second typed edit must remain exactly serializable");
    assert_eq!(decode_wav(&try_encode_wav(&edited).expect("edited boundary snapshot saves")).expect("edited boundary snapshot reopens"), expected);
}

#[semio_framework_async_macros::async_test]
async fn unrepresentable_serialization_states_are_refused_before_writing() {
    let invalid_even_pad = serialization_boundary("invalidEvenPadByte") as u8;
    let cases = [
        WavSnapshot { fmt_pad_byte: invalid_even_pad, ..WavSnapshot::default() },
        WavSnapshot { data_pad_byte: invalid_even_pad, ..WavSnapshot::default() },
        WavSnapshot {
            other_chunks: vec![RiffChunk { fourcc: serialization_boundary_text("validFourcc"), data: vec![1, 2], pad_byte: invalid_even_pad }],
            ..WavSnapshot::default()
        },
        WavSnapshot {
            fmt: WavFmt { ext: Some(vec![0; serialization_boundary("overflowFmtExtensionBytes")]), ..WavFmt::default() },
            ..WavSnapshot::default()
        },
        WavSnapshot {
            other_chunks: vec![RiffChunk { fourcc: serialization_boundary_text("invalidFourcc"), data: vec![1], pad_byte: 0 }],
            ..WavSnapshot::default()
        },
    ];
    let expected_codes = [
        "stdio.wav.serialization.invalid-pad-byte",
        "stdio.wav.serialization.invalid-pad-byte",
        "stdio.wav.serialization.invalid-pad-byte",
        "stdio.wav.serialization.fmt-extension-too-large",
        "stdio.wav.serialization.invalid-fourcc",
    ];
    for (snapshot, code) in cases.iter().zip(expected_codes) {
        let error = validate_wav_serialization(snapshot).expect_err("invalid serialization state must be rejected by schema validation");
        assert_eq!(error.code, code);
        assert!(try_encode_wav(snapshot).expect_err("invalid serialization state must not write bytes").contains(code));
    }
}
