use super::*;

const ONE: CosGrant = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };

fn directory() -> XrefIndex {
    let mut index = XrefIndex::default();
    for (number, entry) in [
        (5, Some(XrefEntry::Compressed { stream_num: 11, index: 2 })),
        (9, Some(XrefEntry::Normal { offset: 600, gen: 0 })),
        (11, Some(XrefEntry::Normal { offset: 69, gen: 0 })),
        (2, Some(XrefEntry::Compressed { stream_num: 11, index: 0 })),
        (3, None),
        (7, Some(XrefEntry::Compressed { stream_num: 11, index: 1 })),
        (1, Some(XrefEntry::Normal { offset: 20, gen: 1 })),
        (8, Some(XrefEntry::Compressed { stream_num: 13, index: 77 })),
    ] {
        index.insert_row(XrefRow { number, entry }, false).unwrap();
    }
    index
}

#[test]
fn file_order_is_physical_with_compressed_members_after_their_container() {
    let index = directory();
    let mut reader = XrefFileOrderCursor::default();
    let mut order = Vec::new();
    let mut allocated = 0;
    let mut complete = false;
    for _ in 0..1000 {
        let step = reader.advance(&index, ONE, false);
        assert!(step.work <= 1);
        allocated += step.allocated_bytes;
        match step.event.unwrap() {
            XrefFileOrderEvent::More => {}
            XrefFileOrderEvent::Object(number) => order.push(number),
            XrefFileOrderEvent::Complete => { complete = true; break; }
            event => panic!("unexpected graph order event {event:?}"),
        }
    }
    assert!(complete);
    assert_eq!(order, [1, 11, 2, 7, 5, 9, 8]);
    assert_eq!(allocated, order.len() * std::mem::size_of::<u32>());
    reader.begin_close();
    assert_eq!(reader.close_step(ONE).released_bytes, allocated);
}

#[test]
fn denied_and_cancelled_order_cursor_leaves_its_original_allocation_for_close() {
    let index = directory();
    let mut reader = XrefFileOrderCursor::default();
    let denied = CosGrant { allocation_bytes: 0, release_bytes: 0, ..ONE };
    let first = reader.advance(&index, denied, false);
    let need = index.len() * std::mem::size_of::<u32>();
    assert_eq!(first.event, Ok(XrefFileOrderEvent::NeedsAllocation { bytes: need }));
    assert_eq!(first.allocated_bytes, 0);
    assert_eq!(reader.advance(&index, ONE, false).allocated_bytes, need);
    assert_eq!(reader.advance(&index, ONE, true).event, Ok(XrefFileOrderEvent::Cancelled));
    reader.begin_close();
    assert_eq!(reader.close_step(denied).event, Ok(XrefFileOrderEvent::NeedsRelease { bytes: need }));
    assert_eq!(reader.close_step(ONE).released_bytes, need);
    assert_eq!(reader.close_step(ONE).event, Ok(XrefFileOrderEvent::Closed));
}
