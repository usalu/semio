//! 🧪️ Successful canonical scratch closure stays in its explicit draining phase until every physical owner closes.
use super::*;
use semio_framework_dsl_record::BorrowedDslRecord;
use semio_framework_value::RetainedCloneGrant;
use crate::original_close::{funded,funded_work,retire};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Source {
    text: String,
    rows: Vec<u64>,
}
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 1048576, maximum_release_bytes: 1048576, maximum_depth: 128 }
}
#[test]
fn successful_scratch_drains_after_complete_changes_to_closing() {
    let source = Source { text: "retained scratch".into(), rows: (0..512).collect() };
    let mut observer = |_| true;
    let mut native = NativeEncodeControl::new(128 * 1048576, &mut observer);
    native.begin_stage(0).unwrap();
    let mut writer = OriginalPackWriting::new("test.original", 1048576).unwrap();
    for _ in 0..2_000_000 {
        if writer.cursor.as_ref().unwrap().is_complete() {
            break;
        }
        funded(grant(), |effect| writer.advance(&source, Source::RECORD, &mut native, effect)).unwrap();
    }
    assert!(writer.cursor.as_ref().unwrap().is_complete());
    assert!(!writer.ready());
    let pointer = writer.wire.as_ptr();
    funded(grant(), |effect| writer.advance(&source, Source::RECORD, &mut native, effect)).unwrap();
    assert!(writer.draining && writer.cursor.is_some());
    assert!(!writer.cursor.as_ref().unwrap().is_complete());
    let mut turns = 0;
    let mut denied_release = false;
    while writer.cursor.is_some() {
        turns += 1;
        assert!(turns < 100000);
        let d = writer.cursor.as_ref().unwrap().retirement_demands().unwrap();
        assert_eq!(writer.demand(65536).unwrap(), d);
        assert_eq!(funded(RetainedCloneGrant { maximum_items: 0, ..grant() }, |effect| writer.advance(&source, Source::RECORD, &mut native, effect)).unwrap(), Default::default());
        assert_eq!(writer.wire.as_ptr(), pointer);
        if d.release_bytes > 0 {
            assert_eq!(funded(RetainedCloneGrant { maximum_release_bytes: 0, ..grant() }, |effect| writer.advance(&source, Source::RECORD, &mut native, effect)).unwrap(), Default::default());
            assert_eq!(writer.demand(65536).unwrap(), d);
            denied_release = true;
        }
        funded(grant(), |effect| writer.advance(&source, Source::RECORD, &mut native, effect)).unwrap();
    }
    assert!(turns > 1 && denied_release && writer.ready());
    for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{let(taken,work)=funded_work(denied,|effect|writer.take(effect));assert!(taken.unwrap().is_none());assert_eq!(work,0);assert_eq!(writer.wire.as_ptr(),pointer);}
    let (taken,work)=funded_work(RetainedCloneGrant{maximum_copy_bytes:0,..grant()},|effect|writer.take(effect));let (wire, progress) = taken.unwrap().unwrap();
    assert_eq!(wire.as_ptr(),pointer);assert_eq!(progress.copied_items,1);assert_eq!(progress.copied_bytes,0);assert_eq!(work,1);
    funded(grant(),|effect|writer.close(effect)).unwrap();assert!(writer.empty());
    retire(wire,grant());

}
