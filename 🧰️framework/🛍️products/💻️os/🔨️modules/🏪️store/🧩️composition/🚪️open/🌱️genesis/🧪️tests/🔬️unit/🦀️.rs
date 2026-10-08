use super::*;
use semio_framework_artifact_reference::ArtifactDialect;

fn reference(value: &serde_json::Value) -> ArtifactRef {
    ArtifactRef { artifact_id: value["artifact_id"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: value["dialect"]["artifact_kind"].as_str().unwrap().into(), standard: value["dialect"]["standard"].as_str().unwrap().into(), subset: value["dialect"]["subset"].as_str().unwrap().into() } }
}

fn grant(copy: usize, capacity: usize, release: usize) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 8 } }

fn close(owner: &mut MemberGenesisEnvelopeEncoder) {
    let bytes = owner.next_close_byte_demand();
    let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant(0, 0, bytes.saturating_sub(1))).unwrap());
    assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
    assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
    assert_eq!(owner.next_close_byte_demand(), bytes);
    let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant(0, 0, bytes)).unwrap());
    let RetainedCloneStep::Complete(progress) = step else { panic!("whole URI release must complete") };
    assert_eq!(progress.released_bytes, bytes);
    assert_eq!((events.requested_bytes, events.released_bytes), (0, bytes));
    assert!(owner.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn member_genesis_envelope_streams_exact_canonical_bytes_with_bounded_work_and_physical_uri_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let maximum = fixture["maximumCopyBytes"].as_u64().unwrap() as usize;
    for row in fixture["cases"].as_array().unwrap() {
        let expected = reference(&row["expected"]);
        let owner = OwnerRef { parent: reference(&row["owner"]["parent"]), slot: row["owner"]["slot"].as_str().unwrap().into(), child_id: row["owner"]["child_id"].as_str().unwrap().into() };
        let segment = row["initialPackHex"].as_str().unwrap().as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        let pack = segment.repeat(row["packRepeats"].as_u64().unwrap() as usize);
        let source = MemberGenesisEnvelopeSource { schema: row["schema"].as_str().unwrap(), expected: &expected, owner: &owner, initial_pack: &pack };
        let canonical = crate::os_store::genesis_member_envelope_pack(source.schema, &expected, &owner, &pack).await.unwrap();
        let mut encoder = MemberGenesisEnvelopeEncoder::new();
        let birth = encoder.next_capacity_byte_demand(source).unwrap();
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant(maximum, birth, 0) }, grant(maximum, birth - 1, 0)] {
            let (progress, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_step(source, &mut [], denied).unwrap());
            assert_eq!(progress, MemberGenesisEnvelopeProgress::default());
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(encoder.next_capacity_byte_demand(source), Some(birth));
        }
        let (progress, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_step(source, &mut [], grant(0, birth, 0)).unwrap());
        assert_eq!(progress.retained_capacity_bytes, birth);
        assert_eq!((events.requested_bytes, events.released_bytes), (birth, 0));
        let mut plan_turns = 0;
        while encoder.encoded_length(source).unwrap().is_none() {
            let (progress, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_step(source, &mut [], grant(maximum, 0, 0)).unwrap());
            assert!(progress.processed_bytes <= maximum && progress.written_bytes == 0);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            plan_turns += 1;
            assert!(plan_turns < 1000);
        }
        assert_eq!(encoder.encoded_length(source).unwrap(), Some(canonical.len()));
        let mut output = vec![0; canonical.len()];
        let mut position = 0;
        let mut turns = 0;
        loop {
            let end = output.len().min(position + maximum);
            let (progress, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| encoder.encode_step(source, &mut output[position..end], grant(maximum, 0, 0)).unwrap());
            assert!(progress.processed_bytes <= maximum && progress.written_bytes <= maximum);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            position += progress.written_bytes;
            turns += 1;
            assert!(turns <= canonical.len().div_ceil(maximum) + 1);
            if progress.complete { break; }
        }
        assert_eq!(position, canonical.len());
        assert_eq!(output, canonical);
        assert_eq!(semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(&output)), row["expectedSha256"].as_str().unwrap(), "independent Node Buffer/noble BLAKE3 complete wire oracle");
        let (decoded_pack, history) = crate::os_store::decode_document_pack_bytes(&output).await.unwrap();
        assert_eq!(decoded_pack, pack);
        let log = crate::os_spr::decode_history(&history, &crate::os_spr::DecodeOptions::default()).await.unwrap();
        assert_eq!((&log.doc_id, &log.schema), (&expected.artifact_id, &source.schema.to_owned()));
        assert!(log.edits.is_empty());
        close(&mut encoder);
        println!("[DEBUG] retained canonical genesis case={} original={} output={} plan-turns={plan_turns} copy-turns={turns} max-copy={maximum} URI-whole-release={birth}", row["id"], pack.len(), output.len());
    }
}

#[test]
fn member_genesis_envelope_cancel_retains_uri_until_one_whole_release_grant() {
    let expected = ArtifactRef { artifact_id: "child".into(), dialect: ArtifactDialect { artifact_kind: "kind".into(), standard: "1".into(), subset: "*".into() } };
    let owner = OwnerRef { parent: expected.clone(), slot: "geometry".into(), child_id: "child".into() };
    let source = MemberGenesisEnvelopeSource { schema: "demo/v1", expected: &expected, owner: &owner, initial_pack: b"original" };
    for turns in [0, 1, 3, 17, 65] {
        let mut encoder = MemberGenesisEnvelopeEncoder::new();
        let birth = encoder.next_capacity_byte_demand(source).unwrap();
        encoder.encode_step(source, &mut [], grant(0, birth, 0)).unwrap();
        for _ in 0..turns { encoder.encode_step(source, &mut [0; 64], grant(64, 0, 0)).unwrap(); }
        close(&mut encoder);
    }
    println!("[DEBUG] retained genesis cancellation at five phases preserves physical URI backing under one-below and frees exact whole extent");
}
