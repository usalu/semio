use super::*;

const PDF: &[u8] = include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
const TURN: CosGrant = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };

#[test]
fn structured_file_budget_covers_footer_revisions_and_offset_validation() {
    let mut damaged = PDF.to_vec();
    let header = PDF.windows(7).position(|bytes| bytes == b"9 0 obj").expect("live object header");
    damaged[header + 2] = b'1';

    for (source, expected_valid) in [(PDF, true), (damaged.as_slice(), false)] {
        let mut cursor = StructuredXrefCursor::new(source.len(), CosLimits::default(), 4096);
        let mut done = false;
        for _ in 0..50_000 {
            let result = cursor.advance(source, TURN, false);
            assert!(result.work <= 1);
            match result.event.unwrap() {
                StructuredXrefEvent::More => {}
                StructuredXrefEvent::Complete { offsets_valid } => {
                    assert_eq!(offsets_valid, expected_valid);
                    let (index, trailer) = cursor.take().unwrap();
                    assert_eq!(trailer.get_inheritable("Info").unwrap().as_ref().unwrap().num, 9);
                    assert!(index.is_free(3));
                    assert!(index.contains_key(&9));
                    done = true;
                    break;
                }
                event => panic!("unexpected structured xref event {event:?}"),
            }
        }
        assert!(done);
    }
}

#[test]
fn structured_file_cancel_retires_owned_revisions_before_index_transfer() {
    let mut cursor = StructuredXrefCursor::new(PDF.len(), CosLimits::default(), 4096);
    let mut owned = 0;
    while owned == 0 {
        let step = cursor.advance(PDF, TURN, false);
        assert!(matches!(step.event, Ok(StructuredXrefEvent::More)));
        owned += step.allocated_bytes - step.released_bytes;
    }
    assert!(matches!(cursor.advance(PDF, TURN, true).event, Ok(StructuredXrefEvent::Cancelled)));
    assert!(cursor.take().is_none());

    let mut released = 0;
    loop {
        let step = cursor.close_step(TURN);
        assert!(step.work <= 1);
        released += step.released_bytes;
        if matches!(step.event, Ok(StructuredXrefEvent::Closed)) { break; }
    }
    assert_eq!(released, owned);
}
