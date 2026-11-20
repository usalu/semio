//! 🧪️ Exercises retained stream output from complete PDF 1.7 stream objects.

use super::*;

const PDF: &[u8] = include_bytes!("🧪️streams.pdf");

fn raw(header: &[u8]) -> &[u8] { raw_from(PDF, header) }

fn raw_from<'a>(pdf: &'a [u8], header: &[u8]) -> &'a [u8] {
    let start = pdf.windows(header.len()).position(|bytes| bytes == header).unwrap() + header.len();
    let count = pdf[start..].iter().position(|byte| !byte.is_ascii_digit()).unwrap();
    let length = std::str::from_utf8(&pdf[start..start + count]).unwrap().parse::<usize>().unwrap();
    let marker = b"\nstream\n";
    let offset = start + pdf[start..].windows(marker.len()).position(|bytes| bytes == marker).unwrap() + marker.len();
    &pdf[offset..offset + length]
}

fn grant(status: StreamDataStatus) -> StreamDataGrant {
    StreamDataGrant {
        work: 1,
        allocation_bytes: if let StreamDataStatus::NeedsAllocation { bytes } = status { bytes } else { 0 },
        release_bytes: if let StreamDataStatus::NeedsRelease { bytes } = status { bytes } else { 0 },
    }
}

fn record(retained: &mut usize, turn: &StreamDataReceipt, allowance: StreamDataGrant) {
    assert!(turn.work <= allowance.work);
    assert!(turn.allocated_bytes <= allowance.allocation_bytes);
    assert!(turn.released_bytes <= allowance.release_bytes);
    *retained += turn.allocated_bytes;
    *retained -= turn.released_bytes;
}

fn close(cursor: &mut PdfStreamDataCursor, mut retained: usize) {
    cursor.begin_close();
    let mut allowance = grant(StreamDataStatus::More);
    for _ in 0..100 {
        let turn = cursor.close_step(allowance);
        record(&mut retained, &turn, allowance);
        let status = turn.status.unwrap();
        if status == StreamDataStatus::Closed { assert_eq!(retained, 0); return; }
        allowance = grant(status);
    }
    panic!("stream backing did not retire");
}

fn verify(mut cursor: PdfStreamDataCursor, source: &[u8], expected: &[u8]) {
    let mut retained = 0;
    let mut allowance = grant(StreamDataStatus::More);
    let mut previous = (0, 0);
    for _ in 0..100_000 {
        let turn = cursor.advance(source, allowance, false);
        record(&mut retained, &turn, allowance);
        let progress = cursor.progress();
        assert!(progress.0 >= previous.0 && progress.1 >= previous.1);
        previous = progress;
        let status = turn.status.unwrap();
        if status == StreamDataStatus::Complete {
            assert_eq!(progress, (source.len(), expected.len()));
            let output = cursor.take_output().unwrap();
            assert_eq!(output, expected);
            assert!(cursor.take_output().is_none());
            close(&mut cursor, retained - output.capacity());
            return;
        }
        assert!(cursor.take_output().is_none());
        allowance = grant(status);
    }
    panic!("stream backing did not complete");
}

#[test]
fn retained_stream_bytes_transfer_only_at_terminal_filter_validation() {
    let plain = b"0 0 m 1 1 l S\n".repeat(192);
    verify(PdfStreamDataCursor::flate(plain.len()), raw(b"4 0 obj\n<< /Length "), &plain);
    verify(PdfStreamDataCursor::identity(plain.len()), &plain, &plain);
    verify(PdfStreamDataCursor::identity(0), b"", b"");
    let samples = [10, 30, 50, 20, 40, 60];
    let cursor = PdfStreamDataCursor::flate_predicted(8, samples.len(), 15, 3, 1, 8).unwrap();
    verify(cursor, raw(b"5 0 obj\n<< /Length "), &samples);
}

#[test]
fn retained_stream_never_transfers_cancelled_or_corrupt_output() {
    let source = raw(b"4 0 obj\n<< /Length ");
    let mut corrupt = source.to_vec();
    *corrupt.last_mut().unwrap() ^= 1;
    for (source, cancel) in [(source, true), (corrupt.as_slice(), false)] {
        let mut cursor = PdfStreamDataCursor::flate(8192);
        let mut retained = 0;
        let mut allowance = grant(StreamDataStatus::More);
        for _ in 0..100_000 {
            let turn = cursor.advance(source, allowance, false);
            record(&mut retained, &turn, allowance);
            assert!(cursor.take_output().is_none());
            match turn.status {
                Ok(_) if cancel && cursor.progress().1 > 500 => {
                    let cancelled = cursor.advance(source, allowance, true);
                    assert_eq!(cancelled.status.unwrap(), StreamDataStatus::Cancelled);
                    assert_eq!(cancelled.work + cancelled.allocated_bytes + cancelled.released_bytes, 0);
                    close(&mut cursor, retained);
                    break;
                }
                Ok(status) => allowance = grant(status),
                Err(_) => {
                    assert!(!cancel && cursor.progress().1 > 500);
                    close(&mut cursor, retained);
                    break;
                }
            }
        }
        assert_eq!(cursor.close_step(grant(StreamDataStatus::More)).status.unwrap(), StreamDataStatus::Closed);
    }
}

#[test]
fn packet_filters_decode_real_streams_under_single_unit_grants() {
    let pdf = include_bytes!("🧪️packet-streams.pdf");
    let text = b"0 0 m 1 1 l S\n";
    verify(PdfStreamDataCursor::ascii_hex(text.len()), raw_from(pdf, b"4 0 obj\n<< /Length "), text);
    let samples = [255, 255, 255, 3, 4, 5];
    verify(PdfStreamDataCursor::run_length(samples.len()), raw_from(pdf, b"5 0 obj\n<< /Length "), &samples);
    verify(PdfStreamDataCursor::ascii_hex(1), b" A >", &[160]);
    let grouped = [0, 0, 0, 0, 20, 30, 40];
    verify(PdfStreamDataCursor::ascii85(grouped.len()), raw_from(pdf, b"6 0 obj\n<< /Length "), &grouped);
    verify(PdfStreamDataCursor::ascii85(grouped.len()), b"<~z'HSY~>", &grouped);
}
