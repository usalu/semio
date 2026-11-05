use crate::{standards::riff_pcm::subsets::any::schema::snapshot::validate_wav_serialization, standards::riff_pcm::subsets::any::schema::snapshot::RiffChunk, standards::riff_pcm::subsets::any::schema::snapshot::WavChunkRef, standards::riff_pcm::subsets::any::schema::snapshot::WavData, standards::riff_pcm::subsets::any::schema::snapshot::WavFmt, standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot};
use crate::apply_mutation;
use crate::standards::riff_pcm::subsets::any::io::*;
use semio_framework_test_fixtures::{fixture_path, fixture_text};

/// 🌱 The artifact's own shipped example (`📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav`), embedded with `include_bytes!` so the test reads exactly the bytes the editor ships.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_fixture() -> Vec<u8> {
    include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav").to_vec()
}

fn serialization_boundary(name: &str) -> usize {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🧭️serialization-boundaries/🔣️.json")).expect("serialization boundary fixture is JSON");
    fixture[name].as_u64().unwrap_or_else(|| panic!("serialization boundary {name:?} is an unsigned integer")) as usize
}

fn serialization_boundary_text(name: &str) -> String {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🧭️serialization-boundaries/🔣️.json")).expect("serialization boundary fixture is JSON");
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

    let max_amp = (0.5 * 32767.0) as i32 as f64;
    let mut max_abs_diff = 0i32;
    for (n, &sample) in samples.iter().enumerate() {
        let reference = ((2.0 * std::f64::consts::PI * 440.0 * n as f64 / 8000.0).sin() * max_amp).round() as i32;
        max_abs_diff = max_abs_diff.max((sample as i32 - reference).abs());
    }
    assert_eq!(max_abs_diff, 0, "decoded samples must exactly match a freshly re-synthesized 440Hz sine");

    let re_encoded = encode_wav(&decoded);
    assert_eq!(re_encoded, fixture, "encode(decode(real fixture)) must be byte-identical");

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
    let snapshot: WavSnapshot = semio_framework_pack_json::from_json_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🧭️preserve-chunk-sequence/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral ordered-chunk fixture decodes");
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
    use crate::standards::riff_pcm::subsets::any::schema::mutations::{WavMutation};

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
    let mut edited = expected.clone();

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
    let outcome = apply_mutation(&mut edited, &WavMutation::SetFmt(crate::standards::riff_pcm::subsets::any::schema::mutations::set_fmt::SetFmt { fmt: expected.fmt.clone() }));
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

/// 🔬️ Runs the independent `ffprobe` binary over `bytes` and returns its exact `key=value` report; a missing `ffprobe` is a hard failure because this cross-check is required.
fn ffprobe_fields(bytes: &[u8], suffix: &str, arguments: &[&str]) -> std::collections::BTreeMap<String, String> {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!("semio-{}-{}.{suffix}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
    std::fs::write(&path, bytes).expect("temporary stream is writable");
    let output = std::process::Command::new("ffprobe").args(["-v", "error", "-of", "default=noprint_wrappers=1"]).args(arguments).arg(&path).output().expect("ffprobe is required for this cross-check and must be installed");
    let _ = std::fs::remove_file(&path);
    assert!(output.status.success(), "ffprobe rejected the stream: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).lines().filter_map(|line| line.split_once('=')).map(|(key, value)| (key.to_string(), value.to_string())).collect()
}

#[test]
fn ffprobe_measures_exactly_the_duration_left_by_a_frame_boundary_trim_and_the_format_survives() {
    let mut trimmed = decode_wav(&real_fixture()).expect("decode real fixture");
    let channels = usize::from(trimmed.fmt.channels);
    let rate = trimmed.fmt.sample_rate;
    let kept_frames = (rate / 2) as usize;
    let WavData::Pcm16(samples) = &mut trimmed.data else { panic!("the real fixture is PCM16") };
    samples.truncate(kept_frames * channels);
    let bytes = encode_wav(&trimmed);
    let reopened = decode_wav(&bytes).expect("trimmed stream reopens");
    assert_eq!(reopened.fmt, trimmed.fmt);
    assert_eq!(reopened.data, trimmed.data);
    let report = ffprobe_fields(&bytes, "wav", &["-show_entries", "format=duration:stream=codec_name,sample_rate,channels"]);
    assert_eq!(report["codec_name"], "pcm_s16le");
    assert_eq!(report["sample_rate"], rate.to_string());
    assert_eq!(report["channels"], channels.to_string());
    let duration: f64 = report["duration"].parse().expect("ffprobe reports a numeric duration");
    assert!((duration - kept_frames as f64 / f64::from(rate)).abs() < 1e-6, "{duration}");
}

#[test]
fn a_sample_edit_retains_the_list_info_chunk_and_its_position_and_ffprobe_still_reads_the_title() {
    let mut edited = decode_wav(&real_fixture()).expect("decode real fixture");
    let mut info = b"INFOINAM".to_vec();
    info.extend_from_slice(&6u32.to_le_bytes());
    info.extend_from_slice(b"Titel\0");
    edited.other_chunks = vec![RiffChunk { fourcc: "LIST".into(), data: info, pad_byte: 0 }];
    edited.chunk_order = vec![WavChunkRef::Format, WavChunkRef::Samples, WavChunkRef::Other(0)];
    validate_wav_serialization(&edited).expect("the tagged document is serializable");
    let WavData::Pcm16(samples) = &mut edited.data else { panic!("the real fixture is PCM16") };
    samples[10] = samples[10].wrapping_add(1000);
    let bytes = encode_wav(&edited);
    let reopened = decode_wav(&bytes).expect("edited stream reopens");
    assert_eq!(reopened.other_chunks, edited.other_chunks);
    assert_eq!(reopened.chunk_order, edited.chunk_order);
    assert_eq!(reopened.data, edited.data);
    let report = ffprobe_fields(&bytes, "wav", &["-show_entries", "format_tags=title"]);
    assert_eq!(report["TAG:title"], "Titel");
}

/// 🔊️ Proves the actual WAV native codec, its compiled protocol identity and publishes the receipt the native-codec projection consumes.
#[test]
fn owned_fixture_publication_reports_canonical_logical_carriers() {
    use semio_os_kernel as store;
    use store::snapshot::{ArtifactDsl, ArtifactPack};
    let snapshot = decode_wav(&real_fixture()).expect("decode real fixture");
    let text = snapshot.print_dsl();
    let binary = snapshot.encode_pack_with(&Default::default()).unwrap();
    assert_eq!(WavSnapshot::parse_dsl(&text).unwrap(), snapshot);
    assert_eq!(WavSnapshot::decode_pack_with(&binary, &Default::default()).unwrap(), snapshot);
    let factories = semio_s_artifact_stdio_wav_io_registry::native_codecs();
    assert_eq!(factories.len(), 1);
    let factory = &factories[0];
    let codec = (factory.codec)();
    let kind = (factory.kind)();
    let compiled = std::fs::read(fixture_path!("../../../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
    let digest = semio_framework_hash::Sha256::digest(&compiled);
    assert_ne!(codec.pack_schema_hash, [0; 32]);
    assert_eq!(codec.pack_schema_hash, digest, "the wav pack identity is the digest of its compiled protocol");
    let definition: serde_json::Value = serde_json::from_str(semio_s_artifact_stdio_wav_io_registry::ARTIFACT_DEFINITION_SCHEMA).expect("wav definition parses");
    assert_eq!(definition["codecs"][0]["native_factory"]["pack_schema_hash"].as_str().expect("declared pack hash"), semio_framework_hash::hex_lower(&codec.pack_schema_hash), "the definition declares exactly the factory's pack hash");
    assert_eq!(kind.id, "s.stdio.wav");
    assert_eq!(codec.schema, "stdio.wav");
    assert_eq!(codec.extension, "semio");
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let receipt = serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});
    eprintln!("native-codec-publication={receipt}");
}

/// 📖️ Every authored grammar and protocol of the diff and mutation facets parses in the semio dialects they are written in.
#[test]
fn the_diff_and_mutation_grammars_and_protocols_parse_in_their_dialects() {
    for (name, source) in [
        ("text diff grammar", include_str!("../../📝️text/🔺️diff/📖️.grammar.semio")),
        ("text mutation grammar", include_str!("../../📝️text/🧬️mutations/📖️.grammar.semio")),
    ] {
        semio_framework_dsl::parse_grammar(source).unwrap_or_else(|error| panic!("{name} must parse: {error:?}"));
    }
    for (name, source) in [
        ("binary diff protocol", include_str!("../../💾️binary/🔺️diff/📡️.protocol.semio")),
        ("binary mutation protocol", include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio")),
    ] {
        semio_framework_dsl::parse_protocol(source).unwrap_or_else(|error| panic!("{name} must parse: {error:?}"));
    }
}
