use super::*;

const ONE: CosGrant = CosGrant { work: 1, allocation_bytes: 256 * 1024, release_bytes: 256 * 1024 };

#[test]
fn normal_projection_borrows_the_original_body_and_accepts_an_exact_length() {
    let source = b"31 2 obj\n<< /Length 40 3 R /Filter /ASCIIHexDecode /Type /XObject >>\nstream\n54657374>\nendstream\nendobj";
    let id = ObjRef { num: 31, gen: 2 };
    let mut cursor = NormalProjectionCursor::new(id, 0, CosLimits::default(), 4096);
    let mut requested = false;
    let mut done = false;
    for _ in 0..10_000 {
        let turn = cursor.advance(source, ONE, false);
        assert!(turn.work <= ONE.work && turn.allocated_bytes <= ONE.allocation_bytes && turn.released_bytes <= ONE.release_bytes);
        match turn.event.unwrap() {
            NormalProjectionEvent::More => {}
            NormalProjectionEvent::NeedsLength(reference) => {
                assert!(!requested);
                assert_eq!(reference, ObjRef { num: 40, gen: 3 });
                assert!(cursor.provide_length(ObjRef { num: 40, gen: 0 }, Some(9)).is_err());
                cursor.provide_length(reference, Some(9)).unwrap();
                requested = true;
            }
            NormalProjectionEvent::Complete => {
                match cursor.take().expect("projected native normal object") {
                    NormalProjection::Value(PdfObject::Stream { dict, data, filters }) => {
                        assert_eq!(data, b"Test");
                        assert_eq!(filters, [super::PdfStreamFilter::AsciiHex]);
                        assert_eq!(dict.len(), 1);
                        assert_eq!(dict[0].value.as_name(), Some("XObject"));
                    }
                    _ => panic!("expected retained stream"),
                }
                done = true;
                break;
            }
            event => panic!("unexpected normal projection event {event:?}"),
        }
    }
    assert!(requested && done);
    cursor.begin_close();
    while cursor.close_step(ONE).event.unwrap() != NormalProjectionEvent::Closed {}
}

#[test]
fn normal_projection_refusals_keep_earlier_dictionary_and_output_backing_owned() {
    let source = b"31 2 obj\n<< /Length 3 /Filter /FutureImage >>\nstream\nabc\nendstream\nendobj";
    let id = ObjRef { num: 31, gen: 2 };
    for cancel in [false, true] {
        let mut cursor = NormalProjectionCursor::new(id, 0, CosLimits::default(), 4096);
        let mut allocated = 0;
        let mut released = 0;
        for _ in 0..5000 {
            let step = cursor.advance(source, ONE, cancel && allocated > 0);
            allocated += step.allocated_bytes;
            released += step.released_bytes;
            match step.event.unwrap() {
                NormalProjectionEvent::More => {}
                NormalProjectionEvent::Cancelled => { assert!(cancel); break; }
                NormalProjectionEvent::Complete => {
                    assert!(!cancel);
                    match cursor.take().unwrap() {
                        NormalProjection::ReferenceStream { dict, span } => {
                            assert_eq!(dict[1].value.as_name(), Some("FutureImage"));
                            assert_eq!(&source[span.start..span.end], b"abc");
                        }
                        _ => panic!("expected authentic reference stream"),
                    }
                    break;
                }
                event => panic!("unexpected normal projection event {event:?}"),
            }
        }
        cursor.begin_close();
        loop {
            let step = cursor.close_step(ONE);
            assert!(step.work <= ONE.work);
            released += step.released_bytes;
            if step.event.unwrap() == NormalProjectionEvent::Closed { break; }
        }
        if cancel { assert_eq!(allocated, released); }
    }
}
