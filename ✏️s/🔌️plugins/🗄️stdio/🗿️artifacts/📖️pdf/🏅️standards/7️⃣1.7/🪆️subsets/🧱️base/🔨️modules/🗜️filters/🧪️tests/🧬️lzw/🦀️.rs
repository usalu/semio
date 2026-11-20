//! 🧪 Drives the genuine PDF LZW streams across dictionary-width changes and predictor grants.

use super::*;

const PDF: &[u8] = include_bytes!("🧪️lzw.pdf");

fn raw(header: &[u8]) -> &[u8] {
    let start = PDF.windows(header.len()).position(|bytes| bytes == header).unwrap() + header.len();
    let digits = PDF[start..].iter().position(|byte| !byte.is_ascii_digit()).unwrap();
    let len = std::str::from_utf8(&PDF[start..start + digits]).unwrap().parse::<usize>().unwrap();
    let token = b"\nstream\n";
    let data = start + PDF[start..].windows(token.len()).position(|bytes| bytes == token).unwrap() + token.len();
    &PDF[data..data + len]
}

fn verify(mut decoder: PdfStreamDataCursor, raw: &[u8], expected: &[u8]) {
    let mut retained = 0;
    let mut grant = StreamDataGrant { work: 1, ..Default::default() };
    for _ in 0..100_000 {
        let receipt = decoder.advance(raw, grant, false);
        assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
        retained += receipt.allocated_bytes;
        retained -= receipt.released_bytes;
        match receipt.status.unwrap() {
            StreamDataStatus::Complete => {
                let data = decoder.take_output().unwrap();
                assert_eq!(data, expected);
                assert!(!raw.is_empty());
                assert_eq!(decoder.progress(), (raw.len(), expected.len()));
                assert!(decoder.take_output().is_none());
                retained -= data.capacity();
                for _ in 0..32 {
                    let turn = decoder.close_step(grant);
                    assert!(turn.work <= 1 && turn.released_bytes <= grant.release_bytes);
                    retained -= turn.released_bytes;
                    match turn.status.unwrap() {
                        StreamDataStatus::Closed => { assert_eq!(retained, 0); return; }
                        StreamDataStatus::NeedsRelease { bytes } => grant.release_bytes = bytes,
                        _ => grant.release_bytes = 0,
                    }
                }
                panic!("LZW stream kept retained bytes");
            }
            StreamDataStatus::NeedsAllocation { bytes } => {
                assert!(decoder.take_output().is_none());
                grant.allocation_bytes = bytes;
                grant.release_bytes = 0;
            }
            StreamDataStatus::NeedsRelease { bytes } => {
                grant.allocation_bytes = 0;
                grant.release_bytes = bytes;
            }
            _ => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
        }
    }
    panic!("LZW stream did not complete");
}

#[test]
fn lzw_decodes_early_and_late_width_changes_reset_and_special_codes() {
    let mut expected = (0..3839).map(|index| (index % 251) as u8).collect::<Vec<_>>();
    expected.extend_from_slice(b"AAAAAA");
    for (early, header) in [
        (false, &b"4 0 obj\n<< /Length "[..]),
        (true, &b"5 0 obj\n<< /Length "[..]),
    ] {
        verify(PdfStreamDataCursor::lzw(expected.len(), early), raw(header), &expected);
    }
}

#[test]
fn lzw_reuses_bounded_png_and_tiff_predictors() {
    let png = raw(b"6 0 obj\n<< /Length ");
    verify(PdfStreamDataCursor::lzw_predicted(8, 6, true, 15, 3, 1, 8).unwrap(), png, &[10, 30, 50, 20, 40, 60]);
    let tiff = raw(b"7 0 obj\n<< /Length ");
    verify(PdfStreamDataCursor::lzw_predicted(6, 6, false, 2, 3, 1, 16).unwrap(), tiff, &[0, 10, 0, 30, 0, 50]);
}

fn finish_raw(decoder: &mut PdfLzwCursor, raw: &[u8], cancelled_at: Option<usize>) {
    let grant = FlateGrant { work: 7, allocation_bytes: 32 * 1024 };
    let mut out = [0; 5];
    loop {
        let step = decoder.advance(raw, &mut out, grant, false);
        assert!(step.work <= grant.work && step.allocated_bytes <= grant.allocation_bytes && step.written_bytes <= out.len());
        if let Some(at) = cancelled_at {
            if decoder.progress().1 >= at {
                let stopped = decoder.advance(raw, &mut out, grant, true);
                assert_eq!(stopped.status.unwrap(), FlateStatus::Cancelled);
                assert_eq!(stopped.work + stopped.allocated_bytes + stopped.written_bytes, 0);
                break;
            }
        }
        match step.status {
            Err(_) if cancelled_at.is_none() => break,
            Ok(FlateStatus::Yield | FlateStatus::NeedsOutput) => {}
            other => panic!("unexpected LZW state {other:?}"),
        }
    }
    let retained = decoder.owned_bytes();
    assert!(retained > 0);
    decoder.begin_close();
    let mut released = 0;
    while !decoder.terminal_is_empty() {
        let demand = decoder.retirement_demand().release_bytes;
        assert_eq!(decoder.close_step(1, demand - 1).work, 0);
        let step = decoder.close_step(1, demand);
        assert_eq!(step.released_bytes, demand);
        assert!(step.work <= 1 && step.released_items <= 1);
        released += step.released_bytes;
    }
    assert_eq!(released, retained);
}

#[test]
fn lzw_cancellation_truncation_and_output_ceilings_retain_backing_until_close() {
    let raw = raw(b"5 0 obj\n<< /Length ");
    finish_raw(&mut PdfLzwCursor::new(3839 + 6, true), raw, Some(3839 + 4));
    finish_raw(&mut PdfLzwCursor::new(3839 + 6, true), &raw[..raw.len() - 1], None);
    finish_raw(&mut PdfLzwCursor::new(3810, true), raw, None);
}
