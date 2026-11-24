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
            assert_eq!(output.to_vec_owner(), expected);
            assert!(cursor.take_output().is_none());
            close(&mut cursor, retained - output.retained_bytes().allocated_bytes());
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

/// 📄️ A 40 KB body is born in separately funded pages: no single allocation exceeds one page's quote, the pages read back across their boundaries, and retiring them frees exactly what was allocated, one page per step.
#[test]
fn identity_stream_output_is_born_and_retired_in_bounded_pages() {
    let body: Vec<u8> = (0..40_000u32).map(|index| (index % 251) as u8).collect();
    let mut cursor = PdfStreamDataCursor::identity(body.len());
    let mut allowance = StreamDataGrant { work: 4096, allocation_bytes: 0, release_bytes: 0 };
    let (mut retained, mut pages, mut largest) = (0usize, 0usize, 0usize);
    let output = loop {
        let turn = cursor.advance(body.as_slice(), allowance, false);
        retained += turn.allocated_bytes;
        retained -= turn.released_bytes;
        largest = largest.max(turn.allocated_bytes);
        pages += usize::from(turn.allocated_bytes != 0);
        match turn.status.unwrap() {
            StreamDataStatus::Complete => break cursor.take_output().unwrap(),
            StreamDataStatus::NeedsAllocation { bytes } => allowance.allocation_bytes = bytes,
            _ => allowance.allocation_bytes = 0,
        }
    };
    assert!(pages >= body.len() / 4096 && largest <= 16 * 1024, "{pages} pages, largest allocation {largest}");
    assert_eq!(retained, output.retained_bytes().allocated_bytes());
    let mut across = vec![0u8; 9000];
    assert_eq!(output.read_bytes(4000, &mut across), 9000);
    assert_eq!(across, body[4000..13_000]);
    let mut output = output;
    let (mut steps, mut released) = (0, 0);
    while !output.retained_bytes().terminal_is_empty() {
        let step = output.cold_retire_page(1 << 20).unwrap();
        assert!(step.needs_release.is_none() && step.released_bytes <= 16 * 1024);
        released += step.released_bytes;
        steps += 1;
    }
    assert_eq!(released, retained);
    assert_eq!(steps, pages + body.len().div_ceil(4096), "one release per page plus one octet discard per leaf page");
    close(&mut cursor, 0);
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
