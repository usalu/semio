//! 🧪 Drives length resolution on a genuine PDF with endstream inside the content bytes.

use super::*;
const PDF: &[u8] = include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
const CONTENT: &[u8] = b"BT /F1 12 Tf 10 70 Td (prefix endstream suffix) Tj ET";

fn offset() -> usize { PDF.windows(8).position(|bytes| bytes == b"4 0 obj\n").unwrap() }

fn next_grant(status: NormalObjectStatus) -> CosGrant {
    CosGrant {
        work: 1,
        allocation_bytes: if let NormalObjectStatus::NeedsAllocation { bytes } = status { bytes } else { 0 },
        release_bytes: if let NormalObjectStatus::NeedsRelease { bytes } = status { bytes } else { 0 },
    }
}

fn account(retained: &mut usize, receipt: &NormalObjectReceipt, grant: CosGrant) {
    assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
    *retained += receipt.allocated_bytes;
    *retained -= receipt.released_bytes;
}

#[test]
fn indirect_length_quotes_once_and_preserves_the_borrowed_range() {
    let mut cursor = PdfNormalObjectCursor::new(offset(), CosLimits::default());
    let mut retained = 0;
    let mut asked = false;
    let mut grant = next_grant(NormalObjectStatus::More);
    for _ in 0..2048 {
        let receipt = cursor.advance(PDF, grant, false);
        account(&mut retained, &receipt, grant);
        let status = receipt.status.unwrap();
        if let NormalObjectStatus::NeedsLength(reference) = status {
            assert!(!asked);
            asked = true;
            assert_eq!(reference, ObjRef { num: 5, gen: 0 });
            let idle = cursor.advance(PDF, grant, false);
            assert_eq!(idle.status, Ok(NormalObjectStatus::AwaitingLength));
            assert_eq!(idle.work + idle.allocated_bytes + idle.released_bytes, 0);
            assert!(cursor.provide_length(ObjRef { num: 5, gen: 1 }, Some(CONTENT.len())).is_err());
            cursor.provide_length(reference, Some(CONTENT.len())).unwrap();
            assert!(cursor.provide_length(reference, Some(0)).is_err());
        }
        if status == NormalObjectStatus::Complete {
            assert!(asked);
            let object = cursor.take_object().unwrap();
            assert_eq!(object.id, ObjRef { num: 4, gen: 0 });
            assert_eq!(object.value.dict_get("Length").unwrap().as_ref(), Some(ObjRef { num: 5, gen: 0 }));
            let span = object.stream.unwrap();
            assert_eq!(&PDF[span.start..span.end], CONTENT);
            assert!(retained < 1024);
            assert!(cursor.take_object().is_none());
            let mut callbacks = 0;
            let (id, value) = super::super::parse_indirect_with_length(PDF, offset(), &mut |reference| {
                callbacks += 1;
                assert_eq!(reference, ObjRef { num: 5, gen: 0 });
                Some(CONTENT.len())
            }).unwrap();
            assert_eq!(callbacks, 1);
            assert_eq!(id, object.id);
            let PdfObject::Stream { data, .. } = value else { panic!("production resolver did not yield a stream"); };
            assert_eq!(data.to_vec_owner(), CONTENT);
            return;
        }
        assert!(cursor.take_object().is_none());
        grant = next_grant(status);
    }
    panic!("indirect stream did not complete");
}

#[test]
fn cancelling_an_outstanding_length_retires_the_frame_within_grants() {
    let mut cursor = PdfNormalObjectCursor::new(offset(), CosLimits::default());
    let mut retained = 0;
    let mut grant = next_grant(NormalObjectStatus::More);
    for _ in 0..2048 {
        let receipt = cursor.advance(PDF, grant, false);
        account(&mut retained, &receipt, grant);
        let status = receipt.status.unwrap();
        if let NormalObjectStatus::NeedsLength(reference) = status {
            let cancelled = cursor.advance(PDF, CosGrant::default(), true);
            assert_eq!(cancelled.status, Ok(NormalObjectStatus::Cancelled));
            assert_eq!(cancelled.work + cancelled.allocated_bytes + cancelled.released_bytes, 0);
            assert!(cursor.provide_length(reference, None).is_err());
            cursor.begin_close();
            break;
        }
        grant = next_grant(status);
    }
    grant = next_grant(NormalObjectStatus::More);
    for _ in 0..2048 {
        let receipt = cursor.close_step(grant);
        account(&mut retained, &receipt, grant);
        let status = receipt.status.unwrap();
        if status == NormalObjectStatus::Closed { assert_eq!(retained, 0); return; }
        grant = next_grant(status);
    }
    panic!("cancelled indirect frame still owns backing");
}
