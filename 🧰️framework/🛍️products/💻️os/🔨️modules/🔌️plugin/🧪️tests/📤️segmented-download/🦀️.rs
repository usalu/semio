//! 📤️ The guest half of the ONE segmented-download chunk contract.
//!
//! Owner: `🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json` +
//! `🧫️fixtures/🔣️.json`. The host mirrors are `SEGMENTED_DOWNLOAD_CONTRACT` (drain + transport) and
//! `SEGMENTED_DOWNLOAD_CHUNK_BYTES` (interpolated into the generated shard worker); these laws hold the
//! PRODUCER's constants equal to the same fixture, so a bound edited in one hop alone fails closed
//! instead of turning a correct producer into a runtime fault on the wire.

use crate::app::{ArtifactOutputChunks, ARTIFACT_OUTPUT_CHUNK_BYTES, ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES};

const CONTRACT_FIXTURE: &str = include_str!("../../../../../../🔨️modules/🎭️actor/📮️shard-client/📤️segmented-download/🧫️fixtures/🔣️.json");

fn contract() -> serde_json::Value {
    serde_json::from_str::<serde_json::Value>(CONTRACT_FIXTURE).expect("the segmented-download contract fixture parses")["contract"].clone()
}

#[test]
fn segmented_download_constants_mirror_the_schema_owned_contract() {
    let contract = contract();
    assert_eq!(contract["chunkBytes"].as_u64(), Some(ARTIFACT_OUTPUT_CHUNK_BYTES as u64), "the producer's per-chunk cap must be the contract's chunkBytes");
    assert_eq!(contract["chunkBytes"].as_u64(), Some(ArtifactOutputChunks::CHUNK_BYTES as u64), "the exported CHUNK_BYTES an app slices by must be the same constant");
    assert_eq!(contract["maximumTotalBytes"].as_u64(), Some(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES as u64), "the producer's total cap must be the contract's maximumTotalBytes");
    assert_eq!(contract["maximumTotalBytes"].as_u64(), Some(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES as u64), "the exported total cap an app refuses against must be the same constant");
    let outstanding = contract["maximumOutstandingChunks"].as_u64().expect("maximumOutstandingChunks");
    assert_eq!(
        outstanding * contract["chunkBytes"].as_u64().expect("chunkBytes"),
        contract["maximumTotalBytes"].as_u64().expect("maximumTotalBytes"),
        "the outstanding-chunk bound must be the total cap divided by the chunk cap, or the drain's loop bound refuses a legal payload"
    );
}

#[test]
fn segmented_download_admits_exactly_the_declared_total_budget() {
    assert!(ArtifactOutputChunks::admit_maximum(0).is_err(), "an empty budget publishes a download that can carry nothing");
    assert_eq!(ArtifactOutputChunks::admit_maximum(1).ok(), Some(1));
    assert_eq!(ArtifactOutputChunks::admit_maximum(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES).ok(), Some(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES));
    let over = ArtifactOutputChunks::admit_maximum(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES + 1).expect_err("an over-cap budget must be refused at construction");
    assert!(format!("{over:?}").contains("segmented-download-total-over-cap"), "the refusal must name the bound it violated, not a generic limit: {over:?}");
}

#[test]
fn segmented_download_slot_table_retires_every_chunk_as_it_is_taken() {
    let payload = vec![7u8; 145_714];
    let chunks = ArtifactOutputChunks::new(ArtifactOutputChunks::admit_maximum(262_144).expect("declared budget"));
    for page in payload.chunks(ArtifactOutputChunks::CHUNK_BYTES) {
        chunks.push(page.to_vec()).expect("every page is within the contract's chunk cap");
    }
    let expected = payload.len().div_ceil(ArtifactOutputChunks::CHUNK_BYTES);
    assert_eq!(chunks.chunks_remaining(), expected, "a 145 714 B payload occupies exactly {expected} outstanding slots");
    assert_eq!(chunks.bytes(), payload.len());
    assert_eq!(chunks.bytes_remaining(), payload.len());
    chunks.seal().expect("seal");
    let mut drained = Vec::new();
    let mut taken = 0usize;
    while let Some(chunk) = chunks.take_chunk().expect("take") {
        taken += 1;
        assert_eq!(chunks.chunks_remaining(), expected - taken, "each taken chunk must RETIRE from the outstanding table, or the table saturates");
        assert_eq!(chunks.bytes_remaining(), payload.len() - drained.len() - chunk.len(), "each taken chunk retires its exact outstanding byte count");
        drained.extend_from_slice(&chunk);
    }
    assert_eq!(taken, expected);
    assert_eq!(chunks.chunks_remaining(), 0, "a fully drained output retains nothing");
    assert_eq!(chunks.bytes_remaining(), 0, "a fully drained output retains no byte authority");
    assert_eq!(drained, payload);
}

#[test]
fn segmented_output_original_full_grant_physical_retirement() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement};
    let contract = contract();
    let chunk_bytes = contract["chunkBytes"].as_u64().unwrap() as usize;
    for copy in [1, 8, 256] {
        let chunks = ArtifactOutputChunks::new(chunk_bytes * 2);
        for capacity in [chunk_bytes, chunk_bytes * 2] {
            let mut bytes = Vec::with_capacity(capacity);
            bytes.extend_from_slice(&[7; 33]);
            chunks.push(bytes).unwrap();
        }
        chunks.seal().unwrap();
        let alias = chunks.clone();
        let download = crate::app::ArtifactDownloadOutput::new("original.semio", "text/plain", None, chunks).unwrap();
        assert!(<crate::app::ArtifactDownloadOutput as RetireOwned>::controlled_retirement_supported());
        let mut owner = match ControlledRetirement::new(download) { Ok(owner) => owner, Err((error, _)) => panic!("original download owner refused: {error}") };
        let birth = owner.next_capacity_byte_demand(copy).unwrap();
        assert_ne!(birth, 0);
        for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: birth - 1, maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() }] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant).unwrap());
            assert_eq!(step.progress(), Default::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(alias.chunks_remaining(), 2);
            assert_eq!(alias.bytes_remaining(), 66);
        }
        for _ in 0..65_536 {
            let ((capacity, release, depth), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| (owner.next_capacity_byte_demand(copy).unwrap(), owner.next_release_byte_demand().unwrap(), owner.next_depth_demand().unwrap()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (heap.requested_bytes, heap.released_bytes));
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(owner.terminal_is_empty());
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(alias.chunks_remaining(), 2);
        let mut owner = match ControlledRetirement::new(alias) { Ok(owner) => owner, Err((error, _)) => panic!("original final queue owner refused: {error}") };
        let mut released = 0;
        for _ in 0..65_536 {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
            if grant.maximum_release_bytes != 0 {
                let denied = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(denied).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (heap.requested_bytes, heap.released_bytes));
            released += heap.released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(owner.terminal_is_empty());
        assert!(released >= chunk_bytes * 3);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original segmented output copy={copy} physicalRelease={released} aliasCustody=retained terminalDrop=0");
    }
}
