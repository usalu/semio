//! 🧪 Exercises the complete controlled section with a genuine incremental PDF.

use super::*;

const SOURCE: &[u8] = include_bytes!("../../../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf");

fn offset() -> usize {
    SOURCE.windows(9).rposition(|bytes| bytes == b"10 0 obj\n").unwrap()
}

fn grant(event: XrefStreamEvent) -> CosGrant {
    CosGrant {
        work: 1,
        allocation_bytes: if let XrefStreamEvent::NeedsAllocation { bytes } = event { bytes } else { 0 },
        release_bytes: if let XrefStreamEvent::NeedsRelease { bytes } = event { bytes } else { 0 },
    }
}

fn account(retained: &mut usize, receipt: &XrefStreamReceipt, grant: CosGrant) {
    assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
    *retained += receipt.allocated_bytes;
    *retained -= receipt.released_bytes;
}

fn close(cursor: &mut XrefStreamSectionCursor, mut retained: usize) {
    cursor.begin_close();
    assert_eq!(cursor.close_step(CosGrant::default()).work, 0);
    let mut allowance = grant(XrefStreamEvent::More);
    for _ in 0..2048 {
        let receipt = cursor.close_step(allowance);
        account(&mut retained, &receipt, allowance);
        let event = receipt.event.unwrap();
        if event == XrefStreamEvent::Closed {
            assert_eq!(retained, 0);
            return;
        }
        allowance = grant(event);
    }
    panic!("xref section still owns allocations");
}

#[test]
fn compressed_xref_section_resumes_at_every_phase_and_preserves_trailer() {
    let mut cursor = XrefStreamSectionCursor::new(offset(), CosLimits::default(), 128, 128);
    let mut rows = Vec::new();
    let mut retained = 0;
    let mut allowance = grant(XrefStreamEvent::More);
    for _ in 0..8192 {
        let receipt = cursor.advance(SOURCE, allowance, false);
        account(&mut retained, &receipt, allowance);
        let event = receipt.event.unwrap();
        if event == XrefStreamEvent::Complete {
            let trailer = cursor.take_trailer().unwrap();
            assert_eq!(trailer.iter().map(|entry| entry.key.as_str()).collect::<Vec<_>>(),
                ["Type", "W", "Index", "Filter", "Length", "Size", "Root", "Info", "Prev"]);
            assert_eq!(trailer[3].value.as_name(), Some("FlateDecode"));
            assert_eq!(trailer[6].value.as_ref().unwrap().num, 1);
            let info = SOURCE.windows(8).position(|bytes| bytes == b"\n9 0 obj").unwrap() + 1;
            assert_eq!(rows, [(0, None), (3, None), (9, Some(XrefEntry::Normal { offset: info, gen: 0 })),
                (10, Some(XrefEntry::Normal { offset: offset(), gen: 0 }))]);
            assert!(cursor.take_trailer().is_none());
            return;
        }
        if let XrefStreamEvent::Row { number, entry } = event { rows.push((number, entry)); }
        assert!(cursor.take_trailer().is_none());
        allowance = grant(event);
    }
    panic!("single-work-grant xref stream did not complete");
}

#[test]
fn xref_section_cancels_and_retires_partial_and_complete_ownership() {
    for stop_after in [10, 120, 280, usize::MAX - 1, usize::MAX] {
        let mut cursor = XrefStreamSectionCursor::new(offset(), CosLimits::default(), 128, 128);
        let mut retained = 0;
        let mut allowance = grant(XrefStreamEvent::More);
        for step in 0..8192 {
            let turn = cursor.advance(SOURCE, allowance, false);
            account(&mut retained, &turn, allowance);
            let event = turn.event.unwrap();
            if step == stop_after || (stop_after == usize::MAX - 1 && matches!(event, XrefStreamEvent::Row { .. })) {
                let turn = cursor.advance(SOURCE, allowance, true);
                assert_eq!(turn.event.unwrap(), XrefStreamEvent::Cancelled);
                assert_eq!(turn.work + turn.allocated_bytes + turn.released_bytes, 0);
                assert!(cursor.take_trailer().is_none());
                break;
            }
            if event == XrefStreamEvent::Complete { break; }
            allowance = grant(event);
        }
        close(&mut cursor, retained);
    }
}

#[test]
fn xref_compatibility_reuses_parsed_trailer_and_borrowed_raw_span() {
    let body = SOURCE.windows(8).rposition(|bytes| bytes == b"\nstream\n").unwrap() + 8;
    let end = body + SOURCE[body..].windows(10).position(|bytes| bytes == b"\nendstream").unwrap();
    let mut pdf = SOURCE[..body].to_vec();
    pdf.extend_from_slice(&SOURCE[body + 2..end - 4]);
    pdf.extend_from_slice(&SOURCE[end..]);

    let mut cursor = XrefStreamSectionCursor::new(offset(), CosLimits::default(), usize::MAX, usize::MAX);
    let grant = CosGrant { work: 128, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    loop {
        match cursor.advance(&pdf, grant, false).event {
            Ok(XrefStreamEvent::More) => {}
            Err(XrefStreamError::Decode(_)) => { assert!(cursor.recover_compatible(&pdf).unwrap()); break; }
            other => panic!("expected format refusal before compatibility recovery, got {other:?}"),
        }
    }
    let mut rows = 0;
    loop {
        match cursor.advance(&pdf, grant, false).event.unwrap() {
            XrefStreamEvent::Row { .. } => rows += 1,
            XrefStreamEvent::More => {}
            XrefStreamEvent::Complete => break,
            other => panic!("unexpected xref compatibility step {other:?}"),
        }
    }
    assert_eq!(rows, 4);
    assert!(cursor.take_trailer().unwrap().iter().any(|entry| entry.key == "Root"));
    cursor.begin_close();
    while cursor.close_step(grant).event.unwrap() != XrefStreamEvent::Closed {}
}
