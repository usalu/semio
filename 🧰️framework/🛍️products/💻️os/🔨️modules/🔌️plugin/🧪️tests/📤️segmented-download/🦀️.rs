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
    assert_eq!(outstanding * contract["chunkBytes"].as_u64().expect("chunkBytes"), contract["maximumTotalBytes"].as_u64().expect("maximumTotalBytes"), "the outstanding-chunk bound must be the total cap divided by the chunk cap, or the drain's loop bound refuses a legal payload");
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
    chunks.seal().expect("seal");
    let mut drained = Vec::new();
    let mut taken = 0usize;
    while let Some(chunk) = chunks.take_chunk().expect("take") {
        taken += 1;
        assert_eq!(chunks.chunks_remaining(), expected - taken, "each taken chunk must RETIRE from the outstanding table, or the table saturates");
        drained.extend_from_slice(&chunk);
    }
    assert_eq!(taken, expected);
    assert_eq!(chunks.chunks_remaining(), 0, "a fully drained output retains nothing");
    assert_eq!(drained, payload);
}
