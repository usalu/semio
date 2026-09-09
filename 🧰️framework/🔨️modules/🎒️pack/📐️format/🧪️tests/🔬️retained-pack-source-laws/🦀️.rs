
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

fn close(cursor: &mut RetainedPackSourceCursor) {
    loop {
        if cursor.close_step(1, RETAINED_PACK_PAGE_BYTES).expect("close") == RetainedPackCloseStep::Complete {
            break;
        }
    }
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

fn source(bytes: &[u8]) -> RetainedPackSourceCursor {
    let pages = bytes.len().div_ceil(RETAINED_PACK_PAGE_BYTES);
    let mut source = RetainedPackSourceCursor::try_new(pages, bytes.len()).expect("source");
    for part in bytes.chunks(RETAINED_PACK_PAGE_BYTES) {
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
    assert!(RetainedPackSourceCursor::try_new(0, 1).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, 0).is_err());
    assert!(RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES + 1).is_err());
    let mut cursor = RetainedPackSourceCursor::try_new(1, RETAINED_PACK_PAGE_BYTES).expect("maximum");
    cursor.admit_page(page(&vec![7; RETAINED_PACK_PAGE_BYTES])).expect("maximum page");
    let rejected = page(&[8]);
    let rejected = cursor.admit_page(rejected).expect_err("maximum plus one");
    assert_eq!(rejected.len(), 1);
    close(&mut cursor);
}

#[test]
fn interruption_resume_is_byte_exact_and_close_is_incremental() {
    let mut cursor = RetainedPackSourceCursor::try_new(2, 8).expect("credits");
    cursor.admit_page(page(b"SPK")).expect("first");
    cursor.admit_page(page(b"123")).expect("second");
    assert_eq!(cursor.grant().expect("unsealed"), None);
    cursor.seal().expect("seal");
    assert_eq!(cursor.grant().expect("S"), Some(RetainedPackSourceEvent::Byte { offset: 0, value: b'S' }));
    assert_eq!(cursor.progress().consumed_bytes, 1);
    assert_eq!(drain(&mut cursor), b"PK123");
    assert_ne!(cursor.close_step(1, RETAINED_PACK_PAGE_BYTES).expect("first close"), RetainedPackCloseStep::Complete);
    close(&mut cursor);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn cancellation_is_observable_and_still_requires_terminal_empty_close() {
    let mut cursor = RetainedPackSourceCursor::try_new(1, 4).expect("credits");
    cursor.admit_page(page(b"SPK1")).expect("page");
    cursor.seal().expect("seal");
    cursor.request_cancel();
    assert_eq!(cursor.grant(), Err("retained-pack.cancelled"));
    close(&mut cursor);
}

#[test]
fn language_neutral_retained_law_ledger_is_complete() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).expect("fixture JSON");
    assert_eq!(fixture["admission"]["pageBytes"], RETAINED_PACK_PAGE_BYTES);
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
