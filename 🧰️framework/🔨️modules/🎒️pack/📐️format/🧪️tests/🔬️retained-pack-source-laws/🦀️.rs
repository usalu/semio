
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

fn close_catalog(cursor: &mut RetainedPackCatalogCursor) -> usize {
    let mut released = 0;
    loop {
        let grant = cursor.next_release_allocation_bytes().expect("catalog release query").unwrap_or(0);
        match cursor.close_step(1, grant).expect("catalog close") {
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

async fn canonical_symbol_pack(symbol: &str) -> Vec<u8> {
    let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec: CodecId(0) };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.expect("symbol writer");
    writer.write_segment(crate::KIND_SYMBOLS, &encode_symbols(&[symbol.to_string()]).await).await.expect("symbol segment");
    writer
        .finish(&Manifest {
            schema_name: symbol.to_string(),
            schema_hash: [7; 32],
            doc_span: ByteRange { offset: 0, len: 0 },
            doc_frame_count: 0,
            symbols_span: ByteRange { offset: 0, len: 0 },
            chunk_table_span: ByteRange { offset: 0, len: 0 },
            field_index_span: ByteRange { offset: 0, len: 0 },
            uncompressed_body_len: 0,
            field_count: 0,
            chunk_count: 0,
            symbol_count: 1,
        })
        .await
        .expect("symbol pack finish")
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
    while let Some(exact) = catalog.next_allocation_bytes().expect("catalog allocation query") {
        let step = catalog.reserve_allocation(exact).expect("catalog exact allocation");
        assert!(step.progressed);
    }
    if let Some(RetainedPackCatalogEvent::DocumentByte { value, .. }) = catalog.grant().expect("catalog grant") {
        document.push(value);
    }
}

fn catalog_segment(kind: u8, raw_len: u64) -> RetainedPackSegmentHeader {
    RetainedPackSegmentHeader { offset: 64, kind, flags: 0, stored_len: raw_len, raw_len, payload_offset: 68 }
}

fn admit_catalog_event(cursor: &mut RetainedPackCatalogCursor, event: RetainedPackSegmentEvent, allocated: &mut usize) -> Result<Option<RetainedPackCatalogEvent>, RetainedPackCatalogFault> {
    cursor.admit(event).expect("retained catalog event admission");
    while let Some(exact) = cursor.next_allocation_bytes()? {
        let step = cursor.reserve_allocation(exact).expect("retained catalog exact allocation");
        assert!(step.progressed && step.allocated_bytes != 0);
        *allocated += step.allocated_bytes;
    }
    cursor.grant()
}

fn catalog_limits(maximum_symbols: u32, maximum_items: u64) -> PackLimits {
    PackLimits { max_file_len: 16 * 1024, max_segment_len: 16 * 1024, max_symbols: maximum_symbols, max_depth: 8, max_items: maximum_items, max_total_alloc: 16 * 1024 }
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
    assert_eq!(fixture["retainedCatalog"]["constructionAllocates"], false);
    assert_eq!(fixture["retainedCatalog"]["symbols"].as_array().expect("catalog symbols").len(), 3);
    assert_eq!(fixture["retainedCatalog"]["grants"]["pendingInputPreserved"], true);
    assert_eq!(fixture["valueTags"].as_array().expect("tags").len(), 24);
    assert!(fixture["hostile"].as_array().expect("hostile laws").iter().any(|law| law == "terminal-empty"));
}

#[test]
fn retained_pack_catalog_exact_utf8_allocation_refusal_and_release_are_conserved() {
    let symbols = ["", "axis", "A€𐍈"];
    let payload = [3, 0, 4, b'a', b'x', b'i', b's', 8, b'A', 0xe2, 0x82, 0xac, 0xf0, 0x90, 0x8d, 0x88];
    let segment = catalog_segment(crate::KIND_SYMBOLS, payload.len() as u64);
    let mut cursor = RetainedPackCatalogCursor::try_new(catalog_limits(3, 1), 3, 12, 7, 0, 64 * 1024).expect("retained catalog credits");
    assert_eq!(cursor.allocated_bytes(), 0);
    let mut allocated = 0;
    assert_eq!(admit_catalog_event(&mut cursor, RetainedPackSegmentEvent::Begin(segment), &mut allocated).expect("symbol begin"), None);
    cursor.admit(RetainedPackSegmentEvent::RawByte { segment, index: 0, value: payload[0] }).expect("symbol count event");
    let exact = cursor.next_allocation_bytes().expect("symbol-span allocation query").expect("symbol-span allocation");
    let before = cursor.progress();
    assert_eq!(cursor.reserve_allocation(exact - 1).expect("subexact allocation refusal"), RetainedPackCatalogAllocationStep::default());
    assert_eq!(cursor.progress(), before);
    while let Some(exact) = cursor.next_allocation_bytes().expect("symbol-span allocation query") {
        let step = cursor.reserve_allocation(exact).expect("symbol-span exact allocation");
        assert!(step.progressed);
        allocated += step.allocated_bytes;
    }
    assert_eq!(cursor.grant().expect("symbol count grant"), None);
    for (index, value) in payload.iter().copied().enumerate().skip(1) {
        admit_catalog_event(&mut cursor, RetainedPackSegmentEvent::RawByte { segment, index: index as u64, value }, &mut allocated).expect("symbol byte");
    }
    admit_catalog_event(&mut cursor, RetainedPackSegmentEvent::Complete { segment, wire_len: payload.len() as u64 + 7 }, &mut allocated).expect("symbol completion");
    assert_eq!(cursor.progress().symbols, 3);
    for (symbol, expected) in symbols.iter().enumerate() {
        let span = cursor.symbol_span(symbol as u64).expect("retained symbol span");
        assert_eq!(span.utf8_len, expected.len() as u64);
        let actual: String = (0..span.scalar_len as usize).map(|index| cursor.symbol_char(symbol as u64, index).expect("retained scalar").expect("retained scalar value")).collect();
        assert_eq!(&actual, expected);
    }
    assert_eq!(cursor.symbol_char(2, 3).expect("past retained scalar"), None);
    let pointer = cursor.retained_symbol_scalar_ptr().expect("retained scalar backing");
    let before = cursor.progress();
    assert_eq!(cursor.close_step(0, usize::MAX).expect("bytes-only close refusal"), RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.progress(), before);
    assert_eq!(cursor.retained_symbol_scalar_ptr(), Some(pointer));
    let released = close_catalog(&mut cursor);
    assert_eq!(released, allocated);
    assert!(cursor.terminal_is_empty());
    eprintln!("[DEBUG] retained-pack-catalog symbols=3 utf8-bytes=12 scalars=7 subexact-allocation-preserved=true bytes-only-close-preserved=true allocated-bytes={allocated} released-bytes={released}");
}

#[test]
fn retained_pack_catalog_limit_and_utf8_faults_are_sticky_until_exact_close() {
    let mut excessive = RetainedPackCatalogCursor::try_new(catalog_limits(2, 2), 2, 8, 8, 2, 64 * 1024).expect("count credits");
    let symbols = catalog_segment(crate::KIND_SYMBOLS, 1);
    let mut allocated = 0;
    admit_catalog_event(&mut excessive, RetainedPackSegmentEvent::Begin(symbols), &mut allocated).expect("count begin");
    excessive.admit(RetainedPackSegmentEvent::RawByte { segment: symbols, index: 0, value: 3 }).expect("maximum plus one pending count");
    let count_fault = excessive.next_allocation_bytes().expect_err("maximum plus one symbol count");
    assert_eq!(count_fault.code, "retained-pack.catalog-symbol-count");
    assert!(excessive.progress().pending_input);
    assert_eq!(excessive.grant().expect_err("sticky count fault"), count_fault);
    assert_eq!(excessive.fault(), Some(count_fault));
    assert_eq!(close_catalog(&mut excessive), allocated);

    let mut cumulative = RetainedPackCatalogCursor::try_new(catalog_limits(2, 1), 2, 7, 7, 0, 64 * 1024).expect("cumulative credits");
    let payload = [2, 4, b'a', b'b', b'c', b'd', 4];
    let symbols = catalog_segment(crate::KIND_SYMBOLS, 11);
    admit_catalog_event(&mut cumulative, RetainedPackSegmentEvent::Begin(symbols), &mut allocated).expect("cumulative begin");
    for (index, value) in payload[..6].iter().copied().enumerate() {
        admit_catalog_event(&mut cumulative, RetainedPackSegmentEvent::RawByte { segment: symbols, index: index as u64, value }, &mut allocated).expect("first individually valid symbol");
    }
    cumulative.admit(RetainedPackSegmentEvent::RawByte { segment: symbols, index: 6, value: payload[6] }).expect("cumulative pending length");
    let cumulative_fault = cumulative.next_allocation_bytes().expect_err("cumulative UTF-8 byte refusal");
    assert_eq!(cumulative_fault.code, "retained-pack.catalog-symbol-bytes");
    assert!(cumulative.progress().pending_input);
    assert_eq!(cumulative.grant().expect_err("sticky cumulative fault"), cumulative_fault);
    let cumulative_allocation = cumulative.allocated_bytes();
    assert_eq!(close_catalog(&mut cumulative), cumulative_allocation);

    let mut malformed = RetainedPackCatalogCursor::try_new(catalog_limits(1, 1), 1, 4, 4, 0, 64 * 1024).expect("UTF-8 credits");
    let symbols = catalog_segment(crate::KIND_SYMBOLS, 6);
    let mut malformed_allocated = 0;
    admit_catalog_event(&mut malformed, RetainedPackSegmentEvent::Begin(symbols), &mut malformed_allocated).expect("UTF-8 begin");
    for (index, value) in [1, 4, b'a', 0xe2].into_iter().enumerate() {
        admit_catalog_event(&mut malformed, RetainedPackSegmentEvent::RawByte { segment: symbols, index: index as u64, value }, &mut malformed_allocated).expect("valid UTF-8 prefix");
    }
    malformed.admit(RetainedPackSegmentEvent::RawByte { segment: symbols, index: 4, value: 0x28 }).expect("malformed continuation pending");
    let utf8_fault = malformed.next_allocation_bytes().expect_err("malformed UTF-8 preview");
    assert_eq!(utf8_fault.code, "retained-pack.catalog-utf8-continuation");
    assert!(malformed.progress().pending_input);
    assert_eq!(malformed.grant().expect_err("sticky UTF-8 fault"), utf8_fault);
    assert_eq!(close_catalog(&mut malformed), malformed_allocated);

    let mut truncated = RetainedPackCatalogCursor::try_new(catalog_limits(1, 1), 1, 4, 4, 0, 64 * 1024).expect("truncated UTF-8 credits");
    let symbols = catalog_segment(crate::KIND_SYMBOLS, 6);
    let mut truncated_allocated = 0;
    admit_catalog_event(&mut truncated, RetainedPackSegmentEvent::Begin(symbols), &mut truncated_allocated).expect("truncated UTF-8 begin");
    for (index, value) in [1, 4, b'a', 0xf0, 0x90].into_iter().enumerate() {
        admit_catalog_event(&mut truncated, RetainedPackSegmentEvent::RawByte { segment: symbols, index: index as u64, value }, &mut truncated_allocated).expect("truncated UTF-8 prefix");
    }
    truncated.admit(RetainedPackSegmentEvent::RawByte { segment: symbols, index: 5, value: 0x8d }).expect("truncated final byte pending");
    assert_eq!(truncated.next_allocation_bytes().expect("truncated byte needs no allocation"), None);
    let truncated_fault = truncated.grant().expect_err("truncated UTF-8 rejection");
    assert_eq!(truncated_fault.code, "retained-pack.catalog-utf8-truncated");
    assert_eq!(truncated.fault(), Some(truncated_fault));
    assert_eq!(close_catalog(&mut truncated), truncated_allocated);

    let mut chunks = RetainedPackCatalogCursor::try_new(catalog_limits(0, 2), 0, 0, 0, 2, 64 * 1024).expect("chunk count credits");
    let mut chunk_allocated = 0;
    for index in 0..2u64 {
        let chunk = RetainedPackSegmentHeader { offset: 100 + index * 8, kind: crate::KIND_CHUNK, flags: 0, stored_len: 1, raw_len: 1, payload_offset: 104 + index * 8 };
        admit_catalog_event(&mut chunks, RetainedPackSegmentEvent::Begin(chunk), &mut chunk_allocated).expect("maximum chunk begin");
        admit_catalog_event(&mut chunks, RetainedPackSegmentEvent::Complete { segment: chunk, wire_len: 8 }, &mut chunk_allocated).expect("maximum chunk complete");
    }
    let third = RetainedPackSegmentHeader { offset: 116, kind: crate::KIND_CHUNK, flags: 0, stored_len: 1, raw_len: 1, payload_offset: 120 };
    chunks.admit(RetainedPackSegmentEvent::Begin(third)).expect("maximum plus one chunk pending");
    let chunk_fault = chunks.next_allocation_bytes().expect_err("maximum plus one chunk count");
    assert_eq!(chunk_fault.code, "retained-pack.catalog-observed-count");
    assert!(chunks.progress().pending_input);
    assert_eq!(close_catalog(&mut chunks), chunk_allocated);
    eprintln!("[DEBUG] retained-pack-catalog sticky-faults=symbol-count,cumulative-utf8,malformed-utf8,truncated-utf8,chunk-count pending-input-preserved=true");
}

#[semio_framework_async_macros::async_test]
async fn retained_pack_catalog_multibyte_scalar_crosses_physical_source_page_and_matches_pack_file() {
    let expected = format!("{}€", "a".repeat(4_056));
    let bytes = canonical_symbol_pack(&expected).await;
    let euro = [0xe2, 0x82, 0xac];
    let position = bytes.windows(euro.len()).position(|window| window == euro).expect("multibyte symbol wire bytes");
    assert_eq!(position % RETAINED_PACK_PAGE_BYTES, RETAINED_PACK_PAGE_BYTES - 1);
    let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 8 * 1024, max_symbols: 1, max_depth: 8, max_items: 1, max_total_alloc: 16 * 1024 };
    let independent = PackFile::open_manifest(bytes.clone(), &limits, VerificationLevel::Full).await.expect("independent full Pack reader");
    assert_eq!(independent.symbol(0).expect("independent schema symbol"), expected);
    let mut source = source(&bytes);
    let mut anchor = RetainedPackAnchorCursor::new();
    let mut segment = RetainedPackSegmentCursor::try_new(limits.clone()).expect("segment cursor");
    let mut catalog = RetainedPackCatalogCursor::try_new(limits, 1, expected.len(), expected.chars().count(), 0, 256 * 1024).expect("catalog cursor");
    let mut document = Vec::new();
    loop {
        let event = source.grant().expect("source grant").expect("sealed source event");
        anchor.grant(Some(event)).expect("anchor collect");
        forward_source(event, &mut segment, &mut catalog, &mut document);
        if matches!(event, RetainedPackSourceEvent::Complete { .. }) {
            break;
        }
    }
    while !anchor.grant(None).expect("anchor verify") {}
    let receipt = catalog.take(anchor.take().expect("anchor handback")).expect("catalog validation").expect("catalog receipt");
    assert_eq!(receipt.manifest.schema_symbol, Some(0));
    let actual: String = (0..catalog.symbol_chars(0).expect("retained scalar count")).map(|index| catalog.symbol_char(0, index).expect("retained scalar").expect("retained scalar value")).collect();
    assert_eq!(actual, expected);
    let allocated = catalog.allocated_bytes();
    anchor.close_step();
    segment.close_step();
    assert_eq!(close_catalog(&mut catalog), allocated);
    close(&mut source);
    eprintln!("[DEBUG] retained-pack-catalog page-crossing-offset={position} scalar={} independent-pack-file=true allocated-bytes={allocated}", u32::from('€'));
}

#[semio_framework_async_macros::async_test]
async fn retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable() {
    let (bytes, expected_document) = canonical_pack(CodecId(1)).await;
    let mut source = source(&bytes);
    let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 4096, max_symbols: 2, max_depth: 8, max_items: 4, max_total_alloc: 4096 };
    let mut anchor = RetainedPackAnchorCursor::new();
    let mut segment = RetainedPackSegmentCursor::try_new(limits.clone()).expect("segment");
    let mut catalog_cursor = RetainedPackCatalogCursor::try_new(limits, 2, 16, 16, 0, 64 * 1024).expect("catalog");
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
    assert_eq!(catalog.manifest.schema_symbol, Some(0));
    let schema: String = (0..catalog_cursor.symbol_chars(0).expect("schema scalar count")).map(|index| catalog_cursor.symbol_char(0, index).expect("schema scalar").expect("schema scalar value")).collect();
    let second: String = (0..catalog_cursor.symbol_chars(1).expect("second scalar count")).map(|index| catalog_cursor.symbol_char(1, index).expect("second scalar").expect("second scalar value")).collect();
    assert_eq!((schema.as_str(), second.as_str()), ("p2d2", "ä"));
    anchor.close_step();
    segment.close_step();
    assert!(close_catalog(&mut catalog_cursor) > 0);
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
    let mut catalog_cursor = RetainedPackCatalogCursor::try_new(limits, 0, 0, 0, 1, 64 * 1024).expect("catalog");
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
    assert_eq!(catalog.manifest.chunk_count, 1);
    assert_eq!(catalog_cursor.chunk(0).expect("retained chunk entry").raw_len, chunk.len() as u64);
    drop(catalog);
    anchor.close_step();
    segment.close_step();
    assert!(close_catalog(&mut catalog_cursor) > 0);
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
