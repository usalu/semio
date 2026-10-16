use semio_framework_pack_error::PackRefusal;
use super::*;
use semio_framework_pack_format::{Manifest, OPTIONAL_CANONICAL, PackWriter, WriteOptions, encode_symbols};

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

fn close_segment(cursor: &mut RetainedPackSegmentCursor) -> usize {
    let mut released = 0;
    loop {
        let grant = cursor.next_release_allocation_bytes().unwrap_or(0);
        match cursor.close_step(1, grant) {
            RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
            RetainedPackCloseStep::Complete => return released,
        }
    }
}

async fn canonical_pack(codec: CodecId) -> (Vec<u8>, Vec<u8>) {
    let options = WriteOptions { required_flags: 0, optional_flags: OPTIONAL_CANONICAL, codec };
    let mut writer = PackWriter::begin(Vec::<u8>::new(), &options).await.expect("writer");
    writer.write_segment(semio_framework_replication_codec::KIND_SYMBOLS, &encode_symbols(&["p2d2".to_string(), "ä".to_string()]).await).await.expect("symbols");
    let document = b"retained-canonical-document".to_vec();
    let doc_offset = writer.position().await;
    writer.write_segment(semio_framework_replication_codec::KIND_DOCUMENT, &document).await.expect("document");
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
    let maximum_allocation_bytes = pages.checked_mul(32 * 1024).expect("test source physical credits");
    let mut source = RetainedPackSourceCursor::try_new(pages, bytes.len(), maximum_allocation_bytes).expect("source");
    for part in bytes.chunks(RETAINED_PACK_PAGE_BYTES) {
        reserve_page(&mut source);
        source.admit_page(page(part)).expect("admit");
    }
    source.seal().expect("seal");
    source
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
    assert_eq!(cursor.grant(), Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled, "retained-pack.cancelled")));
    assert_eq!(close(&mut cursor), admitted);
}

#[test]
fn physical_allocation_refusal_preserves_the_page_producer_and_zero_ledger() {
    let mut cursor = RetainedPackSourceCursor::try_new(1, 1, 1).expect("separate payload and physical limits");
    let required = cursor.next_allocation_bytes().expect_err("physical limit is smaller than metadata backing");
    assert_eq!(required, RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit, "retained-pack.allocation-credits"));
    let producer = page(&[91]);
    let producer = cursor.admit_page(producer).expect_err("allocation refusal returns producer");
    assert_eq!((producer.len(), producer.bytes[0]), (1, 91));
    assert_eq!(cursor.allocated_bytes(), 0);
    assert_eq!(cursor.close_step(1, 0).expect("empty close"), RetainedPackCloseStep::Complete);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn language_neutral_retained_law_ledger_is_complete() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️.json")).expect("fixture JSON");
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
    assert_eq!(fixture["retainedValue"]["recordBody"]["multiLeaf"]["symbols"], 512);
    assert_eq!(fixture["retainedValue"]["recordBody"]["coordinateModel"]["firstUnrepresentableSymbol"], 4_294_967_296u64);
    assert_eq!(fixture["valueTags"].as_array().expect("tags").len(), 24);
    assert!(fixture["hostile"].as_array().expect("hostile laws").iter().any(|law| law == "terminal-empty"));
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
    assert!(matches!(failure, Some(PackRefusal::ChecksumMismatch { segment: "header", .. })));
    anchor.close_step();
    close(&mut source);
}

#[test]
fn retained_pack_pipeline_diagnostic_anchor_is_inline_sticky_and_closes_the_source() {
    let mut source = source(&[0x2a]);
    let allocated = source.allocated_bytes();
    let event = source.grant().expect("source grant").expect("source byte");
    let hostile = match event {
        RetainedPackSourceEvent::Byte { offset, value } => RetainedPackSourceEvent::Byte { offset: offset + 1, value },
        RetainedPackSourceEvent::Complete { .. } => panic!("one-byte source emits its byte first"),
    };
    let mut anchor = RetainedPackAnchorCursor::new();
    let first = anchor.grant(Some(hostile)).expect_err("non-contiguous anchor input");
    assert_eq!(first, PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: 1, detail: "non-contiguous source event" });
    assert_eq!(anchor.grant(Some(event)).expect_err("first anchor fault remains sticky"), first);
    assert_eq!(anchor.close_step(), RetainedPackCloseStep::Complete);
    assert!(anchor.terminal_is_empty());
    assert_eq!(close(&mut source), allocated);
}

fn segment_byte(cursor: &mut RetainedPackSegmentCursor, offset: u64, value: u8) -> Result<Option<RetainedPackSegmentEvent>, PackRefusal> {
    cursor.admit(RetainedPackSourceEvent::Byte { offset, value }).expect("segment byte admission");
    cursor.grant()
}

fn segment_prefix(cursor: &mut RetainedPackSegmentCursor, flags: u8) -> u64 {
    for offset in 0..HEADER_SIZE as u64 {
        assert_eq!(segment_byte(cursor, offset, 0).expect("header byte"), None);
    }
    assert_eq!(segment_byte(cursor, HEADER_SIZE as u64, semio_framework_replication_codec::KIND_DOCUMENT).expect("kind byte"), None);
    assert_eq!(segment_byte(cursor, HEADER_SIZE as u64 + 1, flags).expect("flags byte"), None);
    HEADER_SIZE as u64 + 2
}

#[test]
fn retained_pack_pipeline_diagnostic_segment_and_varint_are_inline_sticky_and_reject_later_ingress() {
    let limits = PackLimits { max_file_len: 512, max_segment_len: 256, max_symbols: 0, max_depth: 8, max_items: 1, max_total_alloc: 512 };
    let mut varint = RetainedPackSegmentCursor::try_new(limits.clone(), 64 * 1024).expect("varint segment");
    let start = segment_prefix(&mut varint, 0);
    for offset in start..start + 9 {
        assert_eq!(segment_byte(&mut varint, offset, 0x80).expect("continued varint"), None);
    }
    let first = segment_byte(&mut varint, start + 9, 0x80).expect_err("overlong retained varint");
    assert_eq!(first, PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "varint", offset: start, detail: "overlong retained varint" });
    assert_eq!(varint.grant().expect_err("first varint fault remains sticky"), first);
    let later = RetainedPackSourceEvent::Byte { offset: start + 10, value: 0 };
    assert_eq!(varint.admit(later).expect_err("faulted segment rejects later ingress"), later);
    close_segment(&mut varint);
    assert!(varint.terminal_is_empty());

    let mut segment = RetainedPackSegmentCursor::try_new(limits, 64 * 1024).expect("reserved-flag segment");
    let stored_len = segment_prefix(&mut segment, 0xf0);
    assert_eq!(segment_byte(&mut segment, stored_len, 0).expect("stored length"), None);
    let first = segment.grant().expect_err("reserved segment flags");
    assert_eq!(first, PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "segment", offset: HEADER_SIZE as u64 + 1, detail: "reserved segment flags are set" });
    assert_eq!(segment.grant().expect_err("first segment fault remains sticky"), first);
    let later = RetainedPackSourceEvent::Byte { offset: stored_len + 1, value: 0 };
    assert_eq!(segment.admit(later).expect_err("faulted segment rejects later ingress"), later);
    close_segment(&mut segment);
    assert!(segment.terminal_is_empty());
}

#[cfg(feature = "deflate")]
#[test]
fn retained_pack_pipeline_diagnostic_deflate_is_inline_sticky_and_closes_after_real_decode() {
    let raw = b"retained diagnostic backing";
    let stored = semio_framework_replication_codec::deflate_compress(raw).expect("cold independent compressor");
    let mut cursor = semio_framework_replication_codec::DeflateRetainedCursor::try_new(raw.len() as u64 + 1, raw.len() as u64 + 1, 64 * 1024).expect("retained inflater");
    let exact = cursor.next_allocation_bytes().expect("retained inflater allocation");
    let allocated = cursor.reserve_allocation(exact).expect("retained inflater reserve").allocated_bytes;
    let mut produced = Vec::new();
    let mut first = None;
    for byte in stored {
        cursor.admit_byte(byte).expect("retained compressed input");
        loop {
            match cursor.grant(false) {
                Ok(semio_framework_replication_codec::DeflateRetainedStep::Byte(byte)) => produced.push(byte),
                Ok(semio_framework_replication_codec::DeflateRetainedStep::NeedInput) => break,
                Ok(semio_framework_replication_codec::DeflateRetainedStep::Complete) => panic!("declared output is deliberately one byte too long"),
                Err(fault) => {
                    first = Some(fault);
                    break;
                }
            }
        }
        if first.is_some() {
            break;
        }
    }
    for _ in 0..raw.len() + 16 {
        if first.is_some() {
            break;
        }
        match cursor.grant(true) {
            Ok(semio_framework_replication_codec::DeflateRetainedStep::Byte(byte)) => produced.push(byte),
            Ok(semio_framework_replication_codec::DeflateRetainedStep::NeedInput) => {}
            Ok(semio_framework_replication_codec::DeflateRetainedStep::Complete) => panic!("declared output is deliberately one byte too long"),
            Err(fault) => first = Some(fault),
        }
    }
    assert_eq!(produced, raw);
    let first = first.expect("declared raw length mismatch");
    assert_eq!(first, PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "deflate", offset: raw.len() as u64, detail: "decompressed length mismatch or trailing input" });
    assert_eq!(cursor.grant(true).expect_err("first inflater fault remains sticky"), first);
    assert_eq!(cursor.admit_byte(0).expect_err("faulted inflater rejects later ingress"), 0);
    let mut released = 0;
    loop {
        let grant = cursor.next_release_allocation_bytes().unwrap_or(0);
        match cursor.close_step(1, grant) {
            semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_bytes, .. } => released += released_bytes,
            semio_framework_replication_codec::RetainedInflateCloseStep::Complete => break,
        }
    }
    assert!(cursor.terminal_is_empty());
    assert_eq!(released, allocated);
}

#[cfg(feature = "deflate")]
async fn drive_segment_physical(bytes: &[u8], expect_allocation: bool) -> (usize, usize, usize, Option<usize>) {
    let limits = PackLimits { max_file_len: bytes.len() as u64, max_segment_len: 64 * 1024, max_symbols: 128, max_depth: 16, max_items: 64 * 1024, max_total_alloc: 1024 * 1024 };
    let mut source = source(bytes);
    let source_allocated = source.allocated_bytes();
    let mut segment = RetainedPackSegmentCursor::try_new(limits, 64 * 1024).expect("physical segment cursor");
    let mut allocated = 0usize;
    let mut compressed_begins = 0usize;
    let mut pointer = None;
    let mut pack_complete = false;
    while !pack_complete {
        let source_event = source.grant().expect("physical source grant").expect("sealed source event");
        segment.admit(source_event).expect("physical segment admission");
        for _ in 0..bytes.len().saturating_mul(4).max(64) {
            if let Some(exact) = segment.next_allocation_bytes() {
                let before = segment.allocated_bytes();
                assert_eq!(segment.reserve_allocation(0).expect("zero inflater grant"), RetainedPackSourceAllocationStep::default());
                assert_eq!(segment.reserve_allocation(exact - 1).expect("subexact inflater grant"), RetainedPackSourceAllocationStep::default());
                assert_eq!(segment.allocated_bytes(), before);
                let step = segment.reserve_allocation(exact).expect("exact inflater allocation");
                allocated += step.allocated_bytes;
                pointer = Some(segment.retained_inflater_ptr().expect("retained inflater pointer"));
            }
            if let Some(event) = segment.grant().expect("physical segment grant") {
                match event {
                    RetainedPackSegmentEvent::Begin(header) if header.flags & 1 != 0 => {
                        compressed_begins += 1;
                        if let Some(expected) = pointer {
                            assert_eq!(segment.retained_inflater_ptr(), Some(expected));
                        }
                    }
                    RetainedPackSegmentEvent::PackComplete { .. } => pack_complete = true,
                    _ => {}
                }
            }
            if pack_complete || segment.preflight().is_ok() {
                break;
            }
        }
    }
    assert_eq!(allocated != 0, expect_allocation);
    let retained = segment.allocated_bytes();
    let stable_pointer = segment.retained_inflater_ptr();
    assert_eq!(retained, allocated);
    let before_zero = (segment.allocated_bytes(), segment.retained_inflater_ptr());
    assert_eq!(segment.close_step(0, 0), RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!((segment.allocated_bytes(), segment.retained_inflater_ptr()), before_zero);
    let released = close_segment(&mut segment);
    assert!(segment.terminal_is_empty());
    assert_eq!(released, allocated);
    assert_eq!(close(&mut source), source_allocated);
    (compressed_begins, allocated, released, stable_pointer)
}

#[cfg(feature = "deflate")]
#[semio_framework_async_macros::async_test]
async fn retained_pack_inflater_physical_identity_has_zero_demand_and_compressed_segments_reuse_exact_backing() {
    let (identity, _) = canonical_pack(CodecId(0)).await;
    let (identity_begins, identity_allocated, identity_released, identity_pointer) = drive_segment_physical(&identity, false).await;
    assert_eq!((identity_begins, identity_allocated, identity_released, identity_pointer), (0, 0, 0, None));

    let (compressed, _) = canonical_pack(CodecId(1)).await;
    let (compressed_begins, allocated, released, pointer) = drive_segment_physical(&compressed, true).await;
    assert!(compressed_begins >= 2, "canonical compressed pack must exercise backing reuse across segments");
    assert!(pointer.is_some());
    assert_eq!(released, allocated);
}

#[cfg(feature = "deflate")]
#[test]
fn retained_pack_inflater_physical_cancellation_retires_pending_input_before_exact_backing() {
    let mut cursor = semio_framework_replication_codec::DeflateRetainedCursor::try_new(1024, 64 * 1024, 64 * 1024).expect("cancellable retained inflater");
    let exact = cursor.next_allocation_bytes().expect("cancellation history demand");
    let allocated = cursor.reserve_allocation(exact).expect("cancellation history reserve").allocated_bytes;
    let pointer = cursor.retained_history_ptr().expect("cancellation pointer");
    cursor.admit_byte(0b11).expect("pending compressed byte");
    assert_eq!(cursor.close_step(0, 0), semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.retained_history_ptr(), Some(pointer));
    assert_eq!(cursor.close_step(1, 0), semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 });
    while cursor.next_release_allocation_bytes().is_none() {
        assert_eq!(cursor.close_step(1, 0), semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 });
    }
    assert_eq!(cursor.close_step(0, allocated - 1), semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.retained_history_ptr(), Some(pointer));
    assert_eq!(cursor.close_step(0, allocated), semio_framework_replication_codec::RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: allocated });
    assert_eq!(cursor.close_step(1, 0), semio_framework_replication_codec::RetainedInflateCloseStep::Complete);
    assert!(cursor.terminal_is_empty());
}
