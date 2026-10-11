use super::*;

fn grant(items: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
}

fn step_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 }
}

/// 🧹️ Closes a job to terminal-empty under an unbounded grant, refusing any refusal receipt.
fn close_job(mut job: impl semio_framework_job::InteractiveJob) {
    semio_framework_job::InteractiveJob::begin_close(&mut job);
    let mut turns = 0usize;
    while !semio_framework_job::InteractiveJob::terminal_is_empty(&job) {
        if let semio_framework_job::InteractiveJobCloseStep::Refused { kind, .. } = semio_framework_job::InteractiveJob::close_step(&mut job, grant(usize::MAX)) {
            panic!("DEFLATE job close was refused: {kind:?}");
        }
        turns += 1;
        assert!(turns < HASH_SIZE + WINDOW + 4_096);
    }
}

#[test]
fn deflate_job_zero_grant_preserves_input_and_one_opportunity_close_is_exact() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
    let mut job = DeflateEncodeJob::new(vec![1, 2, 3], 1);
    let pointer = job.input.as_ptr();
    InteractiveJob::begin_close(&mut job);
    assert_eq!(InteractiveJob::close_step(&mut job, grant(0)), InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
    assert_eq!(job.input.as_ptr(), pointer);
    let mut opportunities = 0usize;
    while !InteractiveJob::terminal_is_empty(&job) {
        let step = InteractiveJob::close_step(&mut job, grant(1));
        if let InteractiveJobCloseStep::Pending { progress } = step {
            assert!(progress.copied_items <= 1);
        }
        opportunities += 1;
        assert!(opportunities < HASH_SIZE + WINDOW + 64);
    }
}

/// 🧳️ A handcrafted OLE compound-file image: the CFB header signature and sector geometry, a FAT
/// sector, a directory sector and stream sectors mixing zero runs, repeated records and a counter —
/// the shape of the embedded `oleObject*.bin` payloads the compact policy is tuned for.
fn compound_file_image() -> Vec<u8> {
    let mut image = vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];
    image.resize(512, 0);
    image[0x18..0x20].copy_from_slice(&[0x3e, 0x00, 0x03, 0x00, 0xfe, 0xff, 0x09, 0x00]);
    let record = b"Root Entry\0semio compound record ";
    for sector in 0..8u32 {
        let mut block = vec![0u8; 512];
        for (index, byte) in block.iter_mut().enumerate() {
            *byte = match sector % 3 {
                0 => (index as u32 * 7 + sector) as u8,
                1 => record[index % record.len()],
                _ => 0,
            };
        }
        image.extend(block);
    }
    image
}

#[test]
fn compact_high_search_embedded_binary_is_standard_deflate() {
    let input = compound_file_image();
    let candidate = deflate_raw_deterministic_compact_high_search(&input).expect("compress compound file");
    assert_eq!(deflate_raw_deterministic_compact_high_search(&input).expect("compress again"), candidate, "the policy is deterministic");
    assert!(candidate.len() < input.len() / 2, "{} of {} bytes", candidate.len(), input.len());
    assert_eq!(inflate_raw(&candidate).expect("inflate"), input);
    assert_eq!(miniz_oxide::inflate::decompress_to_vec(&candidate).expect("miniz_oxide inflates the stream"), input);
}

#[test]
fn adler32_empty_is_one() {
    assert_eq!(adler32(b""), 1);
}

#[test]
fn zlib_round_trip() {
    let payloads: &[&[u8]] = &[b"", b"a", b"hello zlib", &[0u8; 64], b"abracadabra abracadabra"];
    for p in payloads {
        let enc = zlib_compress(p).expect("compress");
        let dec = zlib_decompress(&enc).expect("decompress");
        assert_eq!(&dec, p);
    }
}

#[test]
fn illustrator_partial_flush_materialization_matches_fixture_stream() {
    let expected: &[u8] = include_bytes!("../../../🧫️fixtures/🎨️illustrator-partial-flush.zz");
    let decoded = zlib_decompress(expected).expect("decode Illustrator stream");
    let actual = zlib_compress_illustrator(&decoded).expect("encode Illustrator stream");
    assert_eq!(actual, expected);
}

#[test]
fn raw_deflate_round_trip() {
    let p = b"stdio-deflate-conformance";
    let enc = deflate_raw(p);
    let dec = inflate_raw(&enc).expect("inflate");
    assert_eq!(dec, p);
}

/// 📦️ Flattens a sealed paged `RetainedJobPayload` into the contiguous bytes these byte-identity
/// assertions compare — the job protocol lends pages, never one `Vec<u8>`.
fn retained_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        if let Some(page) = payload.page(index) {
            bytes.extend_from_slice(page);
        }
    }
    assert_eq!(bytes.len(), payload.len(), "retained payload reports {} bytes but its {} page(s) hold {}", payload.len(), payload.page_count(), bytes.len());
    bytes
}

/// ➡️ One scheduler step under a fresh original wallet; `true` once the job lent an outcome.
fn step_once(job: &mut impl semio_framework_job::InteractiveJob, operation: u64, fuel: u64, cancel: semio_framework_job::CancelToken) -> bool {
    use semio_framework_job::{Generation, OperationId, StepBudget, StepContext};
    let mut sequence = 0;
    let mut progress = RetainedCloneProgress::default();
    let mut context = StepContext::new(OperationId(operation), Generation(1), StepBudget::new(fuel, u64::MAX, step_grant()), cancel, || Some(0), &mut sequence, &mut progress);
    job.step(&mut context).expect("DEFLATE job step").is_some()
}

fn drive_encode_job(mut job: DeflateEncodeJob, fuel: u64) -> Vec<u8> {
    let cancel = semio_framework_job::root_cancel_token();
    loop {
        if step_once(&mut job, 1, fuel, cancel.clone()) {
            let output = job.publication.as_ref().and_then(|publication| publication.commit_output()).map(retained_bytes);
            if let Some(output) = output {
                close_job(job);
                return output;
            }
        }
    }
}

#[test]
fn streaming_encode_is_byte_identical_across_batch_sizes() {
    let payload = b"streaming DEFLATE streaming DEFLATE streaming DEFLATE".repeat(64);
    let expected = deflate_raw(&payload);
    for fuel in [1, 2, 7, 64, 1024] {
        assert_eq!(drive_encode_job(DeflateEncodeJob::new(payload.clone(), 29), fuel), expected, "fuel={fuel}");
    }
}

#[test]
fn streaming_encode_matches_pre_refactor_golden_bytes() {
    assert_eq!(deflate_raw(b"stdio-deflate-stream-golden-stdio-deflate-stream-golden"), [43, 46, 73, 201, 204, 215, 77, 73, 77, 203, 73, 44, 73, 213, 45, 46, 41, 74, 77, 204, 213, 77, 207, 207, 73, 73, 205, 211, 197, 35, 7, 0]);
}

#[test]
fn streaming_checkpoint_restore_is_byte_identical() {
    let payload = b"checkpointed owned compression ".repeat(256);
    let expected = deflate_raw(&payload);
    let mut job = DeflateEncodeJob::new(payload, 31);
    let cancel = semio_framework_job::root_cancel_token();
    let (state, applied_progress) = loop {
        if step_once(&mut job, 2, 5, cancel.clone()) {
            let delivered = job.publication.as_ref().and_then(|publication| publication.applied_progress());
            if let Some(applied_progress) = delivered {
                break (job.checkpoint_bytes(), applied_progress);
            }
        }
    };
    assert!(!state.is_empty(), "a delivered checkpoint must own its serialized state");
    let restored = DeflateEncodeJob::from_checkpoint(&state).expect("restore checkpoint");
    assert_eq!(applied_progress as usize, restored.progress().0);
    close_job(job);
    assert_eq!(drive_encode_job(restored, 3), expected);
}

#[test]
fn streaming_encode_observes_cancellation_without_progress() {
    let mut job = DeflateEncodeJob::new(vec![7; 4096], 64);
    let before = job.checkpoint_bytes();
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    assert!(step_once(&mut job, 3, 1, cancel), "a cancelled step lends the cancelled outcome");
    assert_eq!(job.checkpoint_bytes(), before);
    close_job(job);
}

#[test]
fn adversarial_streaming_transition_stays_below_watchdog_ceiling() {
    let mut input = Vec::with_capacity(256 * 1024);
    for index in 0..256 * 1024 {
        input.push(((index * 31) ^ (index >> 5)) as u8);
    }
    let mut job = DeflateEncodeJob::new(input, usize::MAX);
    let cancel = semio_framework_job::root_cancel_token();
    let started = std::time::Instant::now();
    let _ = step_once(&mut job, 4, 1, cancel);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    close_job(job);
}

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the prior
/// `deflate_raw` never searched for LZ77 matches at all -- it emitted every byte as a fixed-
/// Huffman literal (~9 bits/byte average), so compressing ALWAYS expanded the input by ~12.5%.
/// This is the regression test for that: real, repetitive text must come out smaller, not
/// bigger, and must still round-trip exactly.
#[test]
fn raw_deflate_compresses_repetitive_text() {
    let text = "the quick brown fox jumps over the lazy dog. ".repeat(200);
    let p = text.as_bytes();
    let enc = deflate_raw(p);
    assert!(enc.len() < p.len(), "compressed ({}) should be smaller than input ({}) for highly repetitive text", enc.len(), p.len());
    let dec = inflate_raw(&enc).expect("inflate");
    assert_eq!(dec, p);
}

#[test]
fn raw_deflate_round_trips_binary_with_long_range_matches() {
    // A repeating 4-byte pattern well past MIN_MATCH, exercising the hash-chain match finder
    // across many window-fulls (data.len() > WINDOW) so distances near the 32KB boundary are
    // exercised too, not just short-range matches.
    let mut p = Vec::with_capacity(100_000);
    for i in 0..25_000u32 {
        p.extend_from_slice(&i.to_le_bytes());
    }
    let enc = deflate_raw(&p);
    assert!(enc.len() < p.len());
    let dec = inflate_raw(&enc).expect("inflate");
    assert_eq!(dec, p);
}

#[test]
fn raw_deflate_round_trips_random_incompressible_data() {
    // Match search finding nothing (or only sub-MIN_MATCH runs) must still round-trip --
    // pure-literal fallback path.
    let mut state = 0x2545F4914F6CDD1Du64;
    let mut p = Vec::with_capacity(4096);
    for _ in 0..4096 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        p.push((state & 0xFF) as u8);
    }
    let enc = deflate_raw(&p);
    let dec = inflate_raw(&enc).expect("inflate");
    assert_eq!(dec, p);
}

#[test]
fn codec_round_trip() {
    let payload = b"pack-envelope-payload".to_vec();
    let snap = DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: payload.clone() };
    let pack = store::ArtifactPack::encode_pack(&snap);
    let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
    assert_eq!(decoded, snap);
    assert_eq!(decoded.payload, payload);
}

/// 🧪️ `encode_deflate_snapshot`/`decode_deflate_snapshot` round-trip every typed header field,
/// including a preset-dictionary id.
#[test]
fn snapshot_codec_round_trip_with_preset_dictionary() {
    let snap =
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 5, compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0x1234_5678), payload: b"preset-dictionary-id-round-trip".to_vec() };
    let bytes = encode_deflate_snapshot(&snap);
    // 🪆️ FDICT set + DICTID present between CMF/FLG and the deflate body.
    assert_eq!(bytes[1] & 0x20, 0x20);
    let decoded = decode_deflate_snapshot(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

/// 🧪️ Ticket 26/08/10/…: `decode_deflate_snapshot` rejects a CMF/FLG check failure --
/// FCHECK is derived, not fabricated, so a corrupted header must not silently decode.
#[test]
fn snapshot_codec_rejects_bad_check_bits() {
    let mut bytes = encode_deflate_snapshot(&DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: b"corrupt-me".to_vec() });
    bytes[1] ^= 0x01; // flip a FCHECK bit
    assert!(decode_deflate_snapshot(&bytes).is_err());
}
