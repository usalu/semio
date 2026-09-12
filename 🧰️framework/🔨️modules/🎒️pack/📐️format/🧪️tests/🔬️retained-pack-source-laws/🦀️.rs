
use super::*;

fn page(bytes: &[u8]) -> RetainedPackPage {
    let mut owned = [0; RETAINED_PACK_PAGE_BYTES];
    owned[..bytes.len()].copy_from_slice(bytes);
    RetainedPackPage::try_from_array(owned, bytes.len()).expect("valid page")
}

fn drain(cursor: &mut RetainedPackSourceCursor) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        match cursor.grant().expect("grant") {
            Some(RetainedPackSourceEvent::Byte { value, .. }) => bytes.push(value),
            Some(RetainedPackSourceEvent::Complete { .. }) => break,
            None => panic!("sealed cursor must make progress"),
        }
    }
    bytes
}

fn reserve_page(cursor: &mut RetainedPackSourceCursor) -> usize {
    let mut allocated = 0;
    while !cursor.has_reserved_page() {
        let exact = cursor.next_allocation_bytes().expect("next source allocation");
        let step = cursor.reserve_page(exact).expect("exact source allocation");
        assert!(step.progressed);
        assert_eq!(step.allocated_bytes, exact);
        allocated += step.allocated_bytes;
    }
    allocated
}

fn close(cursor: &mut RetainedPackSourceCursor) -> usize {
    let mut released = 0;
    loop {
        let grant = cursor.next_release_allocation_bytes().unwrap_or(0);
        match cursor.close_step(1, grant).expect("close") {
            RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
            RetainedPackCloseStep::Complete => break,
        }
    }
    released
}

async fn canonical_pack(codec: CodecId) -> (Vec<u8>, Vec<u8>) {
    let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.expect("writer");
    writer.write_segment(crate::KIND_SYMBOLS, &encode_symbols(&["p2d2".to_string(), "ä".to_string()]).await).await.expect("symbols");
    let document = b"retained-canonical-document".to_vec();
    let doc_offset = writer.position().await;
    writer.write_segment(crate::KIND_DOCUMENT, &document).await.expect("document");
    let doc_len = writer.position().await - doc_offset;
    let manifest = Manifest {
        schema_name: "p2d2".into(),
        schema_hash: [2; 32],
        doc_span: ByteRange { offset: doc_offset, len: doc_len },
        doc_frame_count: 1,
        symbols_span: ByteRange { offset: 0, len: 0 },
        chunk_table_span: ByteRange { offset: 0, len: 0 },
        field_index_span: ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: document.len() as u64,
        field_count: 1,
        chunk_count: 0,
        symbol_count: 0,
    };
    (writer.finish(&manifest).await.expect("finish"), document)
}

async fn canonical_chunk_pack(chunk: &[u8]) -> Vec<u8> {
    let options = WriteOptions { required_flags: REQUIRED_CHUNKED, optional_flags: OPTIONAL_CANONICAL, codec: CodecId(0) };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.expect("chunk writer");
    assert_eq!(writer.write_chunk(chunk).await.expect("multi-byte chunk"), ChunkId(0));
    writer
        .finish(&Manifest {
            schema_name: String::new(),
            schema_hash: [0; 32],
            doc_span: ByteRange { offset: 0, len: 0 },
            doc_frame_count: 0,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: 0,
            field_count: 0,
            chunk_count: 1,
            symbol_count: 0,
        })
        .await
        .expect("chunk pack finish")
}

fn source(bytes: &[u8]) -> RetainedPackSourceCursor {
    let pages = bytes.len().div_ceil(RETAINED_PACK_PAGE_BYTES);
    let maximum_allocation_bytes = pages.checked_mul(32 * 1024).expect("test source physical credits");
    let mut source = RetainedPackSourceCursor::try_new(pages, bytes.len(), maximum_allocation_bytes).expect("source");
    for part in bytes.chunks(RETAINED_PACK_PAGE_BYTES) {
        reserve_page(&mut source);
        source.admit_page(page(part)).expect("admit");
    }
    source.seal().expect("seal");
    source
}

fn forward_catalog(event: RetainedPackSegmentEvent, catalog: &mut RetainedPackCatalogCursor, document: &mut Vec<u8>) {
    catalog.admit(event).expect("catalog admission");
    if let Some(RetainedPackCatalogEvent::DocumentByte { value, .. }) = catalog.grant().expect("catalog grant") {
        document.push(value);
    }
}

fn forward_source(event: RetainedPackSourceEvent, segment: &mut RetainedPackSegmentCursor, catalog: &mut RetainedPackCatalogCursor, document: &mut Vec<u8>) {
    segment.admit(event).expect("segment admission");
    loop {
        if let Some(event) = segment.grant().expect("segment grant") {
            forward_catalog(event, catalog, document);
        }
        if segment.preflight().is_ok() || matches!(event, RetainedPackSourceEvent::Complete { .. }) {
            break;
        }
    }
}

#[test]
fn zero_maximum_plus_one_and_producer_handback_are_exact() {
    assert!(RetainedPackSourceCursor::try_new(0, 1, 1).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, 0, 1).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, 1, 0).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, 1, usize::MAX).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES + 1, usize::MAX).is_err());
    assert!(RetainedPackSourceCursor::try_new(RETAINED_PACK_MAXIMUM_PAGES + 1, RETAINED_PACK_PAGE_BYTES, isize::MAX as usize).is_err());
    let mut cursor = RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES, 32 * 1024).expect("maximum");
    let required = cursor.next_allocation_bytes().expect("metadata allocation bytes");
    assert_eq!(cursor.reserve_page(required - 1).expect("insufficient allocation grant"), RetainedPackSourceAllocationStep::default());
    assert_eq!(cursor.allocated_bytes(), 0);
    let rejected = page(&[6]);
    let rejected = cursor.admit_page(rejected).expect_err("missing physical page allocation");
    assert_eq!(rejected.bytes[0], 6);
    let admitted = reserve_page(&mut cursor);
    cursor.admit_page(page(&vec![7; RETAINED_PACK_PAGE_BYTES])).expect("maximum page");
    let rejected = page(&[8]);
    let rejected = cursor.admit_page(rejected).expect_err("maximum plus one");
    assert_eq!(rejected.len(), 1);
    assert_eq!(close(&mut cursor), admitted);
}

#[test]
fn interruption_resume_is_byte_exact_and_close_is_incremental() {
    let mut cursor = RetainedPackSourceCursor::try_new(2, 8, 64 * 1024).expect("credits");
    let mut admitted = reserve_page(&mut cursor);
    cursor.admit_page(page(b"SPK")).expect("first");
    admitted += reserve_page(&mut cursor);
    cursor.admit_page(page(b"123")).expect("second");
    assert_eq!(cursor.grant().expect("unsealed"), None);
    cursor.seal().expect("seal");
    let before_zero_grant = cursor.progress();
    assert_eq!(cursor.close_step(0, 0).expect("zero close grant"), RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.progress(), before_zero_grant);
    assert_eq!(cursor.grant().expect("S"), Some(RetainedPackSourceEvent::Byte { offset: 0, value: b'S' }));
    assert_eq!(cursor.progress().consumed_bytes, 1);
    assert_eq!(drain(&mut cursor), b"PK123");
    assert_eq!(cursor.close_step(1, 0).expect("logical close"), RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
    let pointer = cursor.retained_page_ptr(1).expect("retained tail-page backing");
    let progress = cursor.progress();
    assert_eq!(cursor.initialized_pages(), 1);
    assert_eq!(progress.allocated_bytes, admitted);
    assert!(std::mem::size_of::<RetainedPackPage>() > RETAINED_PACK_PAGE_BYTES);
    assert_eq!(cursor.close_step(1, 0).expect("second logical close"), RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(cursor.initialized_pages(), 0);
    let exact = cursor.next_release_allocation_bytes().expect("exact payload release");
    assert_eq!(exact, std::mem::size_of::<RetainedPackPage>());
    assert_eq!(cursor.close_step(1, exact - 1).expect("insufficient close"), RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.retained_page_ptr(1), Some(pointer));
    assert_eq!(cursor.progress().reserved_pages, progress.reserved_pages);
    assert_eq!(cursor.allocated_bytes(), admitted);
    assert_eq!(close(&mut cursor), admitted);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn cancellation_is_observable_and_still_requires_terminal_empty_close() {
    let mut cursor = RetainedPackSourceCursor::try_new(1, 4, 32 * 1024).expect("credits");
    let admitted = reserve_page(&mut cursor);
    cursor.admit_page(page(b"SPK1")).expect("page");
    cursor.seal().expect("seal");
    assert!(matches!(cursor.grant(), Ok(Some(RetainedPackSourceEvent::Byte { offset: 0, value: b'S' }))));
    cursor.request_cancel();
    assert_eq!(cursor.grant(), Err("retained-pack.cancelled"));
    assert_eq!(close(&mut cursor), admitted);
    eprintln!("[DEBUG] retained-pack-source cancelled-after-byte=true admitted-allocation-bytes={admitted} released-allocation-bytes={admitted}");
}

#[test]
fn physical_allocation_refusal_preserves_the_page_producer_and_zero_ledger() {
    let mut cursor = RetainedPackSourceCursor::try_new(1, 1, 1).expect("separate payload and physical limits");
    let required = cursor.next_allocation_bytes().expect_err("physical limit is smaller than metadata backing");
    assert_eq!(required, "retained-pack.allocation-credits");
    let producer = page(&[91]);
    let producer = cursor.admit_page(producer).expect_err("allocation refusal returns producer");
    assert_eq!((producer.len(), producer.bytes[0]), (1, 91));
    assert_eq!(cursor.allocated_bytes(), 0);
    assert_eq!(cursor.close_step(1, 0).expect("empty close"), RetainedPackCloseStep::Complete);
    assert!(cursor.terminal_is_empty());
    eprintln!("[DEBUG] retained-pack-source allocation-refusal producer-preserved=true allocation-ledger=0 terminal=true");
}

#[test]
fn language_neutral_retained_law_ledger_is_complete() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).expect("fixture JSON");
    assert_eq!(fixture["admission"]["pageBytes"], RETAINED_PACK_PAGE_BYTES);
    assert!(std::mem::size_of::<RetainedPackPage>() >= fixture["physicalOwnership"]["pageItemMinimumBytes"].as_u64().unwrap() as usize);
    assert_eq!(fixture["physicalOwnership"]["zeroGrant"]["mutates"], false);
    assert_eq!(fixture["physicalOwnership"]["logicalPop"]["releasedBytes"], 0);
    assert!(fixture["admission"]["cases"].as_array().unwrap().iter().any(|value| value == "allocator-overgrant-sticky"));
    assert_eq!(fixture["multiByteChunk"]["rawByteEvents"], fixture["multiByteChunk"]["bytes"].as_array().unwrap().len());
    assert_eq!(fixture["multiByteChunk"]["observations"], 1);
    assert_eq!(fixture["valueTags"].as_array().expect("tags").len(), 24);
    assert!(fixture["hostile"].as_array().expect("hostile laws").iter().any(|law| law == "terminal-empty"));
}

#[semio_framework_async_macros::async_test]
async fn retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable() {
    let (bytes, expected_document) = canonical_pack(CodecId(1)).await;
    let mut source = source(&bytes);
    let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 4096, max_symbols: 2, max_depth: 8, max_items: 4, max_total_alloc: 4096 };
    let mut anchor = RetainedPackAnchorCursor::new();
    let mut segment = RetainedPackSegmentCursor::try_new(limits.clone()).expect("segment");
    let mut catalog_cursor = RetainedPackCatalogCursor::try_new(limits, 2, 0, 16).expect("catalog");
    let mut document = Vec::new();
    loop {
        let event = source.grant().expect("source grant").expect("sealed source event");
        anchor.grant(Some(event)).expect("anchor collect");
        forward_source(event, &mut segment, &mut catalog_cursor, &mut document);
        if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
            break;
        }
    }
    while !anchor.grant(None).expect("anchor verify") {}
    let superblock = anchor.take().expect("anchor handback");
    let catalog = catalog_cursor.take(superblock).expect("catalog result").expect("catalog handback");
    assert_eq!(document, expected_document);
    assert_eq!(catalog.manifest.schema_name, "p2d2");
    assert_eq!(catalog.symbols, ["p2d2", "ä"]);
    anchor.close_step();
    segment.close_step();
    assert_eq!(catalog_cursor.close_step(1), RetainedPackCloseStep::Complete);
    close(&mut source);
}

#[semio_framework_async_macros::async_test]
async fn multi_byte_chunk_is_observed_once_at_begin_and_matches_pack_file() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).expect("fixture JSON");
    let chunk: Vec<u8> = fixture["multiByteChunk"]["bytes"].as_array().expect("chunk bytes").iter().map(|value| value.as_u64().expect("byte") as u8).collect();
    assert!(chunk.len() > 1);
    let bytes = canonical_chunk_pack(&chunk).await;
    let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 4096, max_symbols: 0, max_depth: 8, max_items: 4, max_total_alloc: 4096 };
    let independent = PackFile::open_manifest(bytes.clone(), &limits, VerificationLevel::Full).await.expect("independent Pack reader");
    assert_eq!(independent.chunk_count(), fixture["multiByteChunk"]["observations"].as_u64().unwrap());
    assert_eq!(independent.read_chunk(ChunkId(0), VerificationLevel::Full).await.expect("independent chunk"), chunk);
    let mut source = source(&bytes);
    let mut anchor = RetainedPackAnchorCursor::new();
    let mut segment = RetainedPackSegmentCursor::try_new(limits.clone()).expect("segment");
    let mut catalog_cursor = RetainedPackCatalogCursor::try_new(limits, 0, 1, 0).expect("catalog");
    let mut document = Vec::new();
    loop {
        let event = source.grant().expect("source grant").expect("sealed source event");
        anchor.grant(Some(event)).expect("anchor collect");
        forward_source(event, &mut segment, &mut catalog_cursor, &mut document);
        if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
            break;
        }
    }
    while !anchor.grant(None).expect("anchor verify") {}
    let catalog = catalog_cursor.take(anchor.take().expect("anchor handback")).expect("catalog result").expect("catalog handback");
    assert_eq!(catalog.chunks.len(), 1);
    assert_eq!(catalog.chunks[0].raw_len, chunk.len() as u64);
    drop(catalog);
    anchor.close_step();
    segment.close_step();
    assert_eq!(catalog_cursor.close_step(1), RetainedPackCloseStep::Complete);
    close(&mut source);
    eprintln!("[DEBUG] retained-pack-catalog multi-byte-chunk-bytes={} begin-observations=1 independent-reader-match=true", chunk.len());
}

#[semio_framework_async_macros::async_test]
async fn retained_anchor_rejects_hostile_crc_and_requires_explicit_close() {
    let (mut bytes, _) = canonical_pack(CodecId(0)).await;
    bytes[20] ^= 1;
    let mut source = source(&bytes);
    let mut anchor = RetainedPackAnchorCursor::new();
    loop {
        let event = source.grant().expect("source").expect("event");
        anchor.grant(Some(event)).expect("collection cannot fail before verification");
        if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
            break;
        }
    }
    let mut failure = None;
    while failure.is_none() {
        failure = anchor.grant(None).err();
    }
    assert!(matches!(failure, Some(PackError::ChecksumMismatch { segment: "header", .. })));
    anchor.close_step();
    close(&mut source);
}
