
use super::*;

#[test]
fn deflate_job_zero_grant_preserves_input_and_one_opportunity_close_is_exact() {
    let mut job = DeflateEncodeJob::new(vec![1, 2, 3], 1);
    let pointer = job.input.as_ptr();
    semio_framework_job::InteractiveJob::begin_close(&mut job);
    assert_eq!(semio_framework_job::InteractiveJob::close_step(&mut job, 0, usize::MAX), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(job.input.as_ptr(), pointer);
    let mut opportunities = 0usize;
    while !semio_framework_job::InteractiveJob::terminal_is_empty(&job) {
        let step = semio_framework_job::InteractiveJob::close_step(&mut job, 1, usize::MAX);
        if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } = step {
            assert!(released_items <= 1);
        }
        opportunities += 1;
        assert!(opportunities < HASH_SIZE + WINDOW + 64);
    }
}

fn raw_zip_member<'a>(archive: &'a [u8], wanted: &str) -> Option<&'a [u8]> {
    let mut offset = 0usize;
    while archive.get(offset..offset + 4) == Some(b"PK\x03\x04") {
        let compressed = u32::from_le_bytes(archive[offset + 18..offset + 22].try_into().ok()?) as usize;
        let name_len = u16::from_le_bytes(archive[offset + 26..offset + 28].try_into().ok()?) as usize;
        let extra_len = u16::from_le_bytes(archive[offset + 28..offset + 30].try_into().ok()?) as usize;
        let name_start = offset + 30;
        let payload_start = name_start + name_len + extra_len;
        if std::str::from_utf8(&archive[name_start..name_start + name_len]).ok()? == wanted {
            return Some(&archive[payload_start..payload_start + compressed]);
        }
        offset = payload_start + compressed;
    }
    None
}

#[test]
fn exact_pptx_bin_policy() {
    let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
    for path in ["ppt/embeddings/oleObject1.bin", "ppt/embeddings/oleObject2.bin", "ppt/embeddings/oleObject3.bin"] {
        let expected = raw_zip_member(&archive, path).expect("fixture OLE");
        let input = inflate_raw(expected).expect("inflate fixture OLE");
        let candidate = deflate_raw_deterministic_compact_high_search(&input).expect("compress fixture OLE");
        assert_eq!(candidate, expected, "embedded binary policy must reproduce {path}");
    }
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
    let fixture = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../temp/📄️bachelor-thesis.pdf")).expect("fixture");
    let marker = b"/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
    let start = fixture.windows(marker.len()).position(|window| window == marker).expect("Illustrator stream") + marker.len();
    let expected = &fixture[start..start + 3362];
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

/// 📦️ Flattens a paged `RetainedJobPayload` back into the contiguous bytes these byte-identity
/// assertions compare, then closes it to terminal-empty — the job protocol hands out pages, never
/// one `Vec<u8>`, and `RetainedJobPayload::drop` refuses any payload that still owns page backing.
fn retained_bytes(mut payload: semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        if let Some(page) = payload.page(index) {
            bytes.extend_from_slice(page);
        }
    }
    assert_eq!(bytes.len(), payload.len(), "retained payload reports {} bytes but its {} page(s) hold {}", payload.len(), payload.page_count(), bytes.len());
    while payload.close_step(usize::MAX, usize::MAX) != semio_framework_job::JobPayloadCloseStep::Complete {}
    bytes
}

fn drive_encode_job(mut job: DeflateEncodeJob, fuel: u64) -> Vec<u8> {
    use semio_framework_job::{Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome, root_cancel_token};
    let cancel = root_cancel_token();
    let mut sequence = 0;
    loop {
        let mut context = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, u64::MAX), cancel.clone(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(commit) => return retained_bytes(commit.output),
            StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
            outcome => panic!("unexpected DEFLATE job outcome: {outcome:?}"),
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
    use semio_framework_job::{Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome, root_cancel_token};
    let payload = b"checkpointed owned compression ".repeat(256);
    let expected = deflate_raw(&payload);
    let mut job = DeflateEncodeJob::new(payload, 31);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut context = StepContext::new(OperationId(2), Generation(1), StepBudget::new(5, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break checkpoint;
        }
    };
    assert!(!checkpoint.state.is_empty(), "a CheckpointReady outcome must own its serialized state; an empty retained payload means `payload_from_bytes` rejected it and `retained_payload` swallowed the rejection");
    let restored = DeflateEncodeJob::from_checkpoint(&retained_bytes(checkpoint.state)).expect("restore checkpoint");
    assert_eq!(checkpoint.applied_progress as usize, restored.progress().0);
    assert_eq!(drive_encode_job(restored, 3), expected);
}

#[test]
fn streaming_encode_observes_cancellation_without_progress() {
    use semio_framework_job::{Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome, root_cancel_token};
    let mut job = DeflateEncodeJob::new(vec![7; 4096], 64);
    let before = job.checkpoint_bytes();
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(3), Generation(1), StepBudget::new(1, u64::MAX), cancel, || Some(0), &mut sequence);
    assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(job.checkpoint_bytes(), before);
}

#[test]
fn adversarial_streaming_transition_stays_below_watchdog_ceiling() {
    use semio_framework_job::{Generation, InteractiveJob, OperationId, StepBudget, StepContext, root_cancel_token};
    let mut input = Vec::with_capacity(256 * 1024);
    for index in 0..256 * 1024 {
        input.push(((index * 31) ^ (index >> 5)) as u8);
    }
    let mut job = DeflateEncodeJob::new(input, usize::MAX);
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(4), Generation(1), StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
    let started = std::time::Instant::now();
    let _ = job.step(&mut context);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
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
