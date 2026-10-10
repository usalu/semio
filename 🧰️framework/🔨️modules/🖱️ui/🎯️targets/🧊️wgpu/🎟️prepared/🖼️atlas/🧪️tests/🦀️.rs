use super::*;
use semio_framework_trace::{observe_heap_allocations_on_this_thread, HeapAllocationObservation};

fn fixture_grant(row: &serde_json::Value) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: row["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: row["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: row["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize }
}

fn fixture_owner(row: &serde_json::Value, source: &[u8], authority: PreparedAtlasAuthority) -> PreparedAtlasPages {
    let ((mut pages, progress), heap) = observe_heap_allocations_on_this_thread(|| PreparedAtlasPages::try_new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, row["channels"].as_u64().unwrap() as u8, source.len(), authority).unwrap());
    assert!(progress.fits(authority.normal));
    assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
    assert!(!heap.overflowed);
    let (progress, heap) = observe_heap_allocations_on_this_thread(|| pages.admit_original_slots(authority.normal).unwrap());
    assert!(progress.fits(authority.normal));
    assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
    assert!(!heap.overflowed);
    while pages.next_row() < pages.height() {
        let ((_, progress), heap) = observe_heap_allocations_on_this_thread(|| pages.push_page(source, pages.next_row(), authority.normal).unwrap());
        assert!(progress.fits(authority.normal));
        assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
        assert!(!heap.overflowed);
    }
    assert_eq!(pages.len(), row["pages"].as_array().unwrap().len());
    for (index, expected) in row["pages"].as_array().unwrap().iter().enumerate() {
        let (bytes, start, rows) = pages.page(index).unwrap();
        assert_eq!((start, rows, bytes.len()), (expected["startRow"].as_u64().unwrap() as u32, expected["rows"].as_u64().unwrap() as u32, expected["payloadBytes"].as_u64().unwrap() as usize));
        assert!(bytes.iter().all(|byte| *byte == row["sourceByte"].as_u64().unwrap() as u8));
    }
    pages
}

#[test]
fn atlas_original_metadata_undergrant_retains_every_original_page_and_backing() {
    let _guard = super::tests::prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let authority = PreparedAtlasAuthority { normal: fixture_grant(&fixture["authority"]["normal"]), retirement: fixture_grant(&fixture["authority"]["retirement"]) };
    for row in fixture["cases"].as_array().unwrap() {
        let source = vec![row["sourceByte"].as_u64().unwrap() as u8; row["sourceBytes"].as_u64().unwrap() as usize];
        let mut pages = fixture_owner(row, &source, authority);
        let expected = pages.original_owner_progress().retained_capacity_bytes;
        let mut release = 0;
        for _ in 0..64 {
            if pages.terminal_is_empty() { break; }
            let owner = pages.owner.as_ref().map(|owner| &**owner as *const _);
            let backing = pages.owner.as_ref().and_then(|owner| owner.slots.as_ref().map(|slots| &**slots as *const _));
            let last = pages.len().checked_sub(1).and_then(|index| pages.page(index).map(|page| page.0.as_ptr()));
            let len = pages.len();
            for denied in [RetainedCloneGrant { maximum_copy_bytes: 0, ..authority.retirement }, RetainedCloneGrant { maximum_items: 0, ..authority.retirement }, RetainedCloneGrant { maximum_depth: 0, ..authority.retirement }] {
                let (step, heap) = observe_heap_allocations_on_this_thread(|| pages.close_original_step(denied));
                assert!(matches!(step, CloseStep::Pending { progress } if progress == Default::default()) || matches!(step, CloseStep::Refused { kind: ValueRefusalKind::DepthLimit, progress } if progress == Default::default()));
                assert_eq!(heap, HeapAllocationObservation::default());
                assert_eq!(pages.len(), len);
                assert_eq!(pages.owner.as_ref().map(|owner| &**owner as *const _), owner);
                assert_eq!(pages.owner.as_ref().and_then(|owner| owner.slots.as_ref().map(|slots| &**slots as *const _)), backing);
                assert_eq!(pages.len().checked_sub(1).and_then(|index| pages.page(index).map(|page| page.0.as_ptr())), last);
            }
            let quoted = pages.owner.as_ref().unwrap().original_close_progress();
            let (step, heap) = observe_heap_allocations_on_this_thread(|| pages.close_original_step(authority.retirement));
            assert_eq!(step.progress(), quoted);
            assert!(step.progress().fits(authority.retirement));
            assert_eq!((quoted.retained_capacity_bytes, quoted.released_bytes), (heap.requested_bytes, heap.released_bytes));
            assert_eq!(heap.largest_release_bytes, heap.released_bytes);
            assert!(!heap.overflowed);
            release += heap.released_bytes;
        }
        assert!(pages.terminal_is_empty());
        assert_eq!(release, expected);
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(pages));
        assert_eq!(heap, HeapAllocationObservation::default());
    }
}

#[test]
fn atlas_abandonment_publishes_the_pre_admitted_original_header_without_heap_effects() {
    let _guard = super::tests::prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let authority = PreparedAtlasAuthority { normal: fixture_grant(&fixture["authority"]["normal"]), retirement: fixture_grant(&fixture["authority"]["retirement"]) };
    for row in fixture["cases"].as_array().unwrap() {
        let source = vec![row["sourceByte"].as_u64().unwrap() as u8; row["sourceBytes"].as_u64().unwrap() as usize];
        let pages = fixture_owner(row, &source, authority);
        let slot = usize::from(pages.abandonment_slot);
        let original = &**pages.owner.as_ref().unwrap() as *const PreparedAtlasAbandonment as *mut PreparedAtlasAbandonment;
        let expected = pages.original_owner_progress().retained_capacity_bytes;
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(pages));
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
        let mut release = 0;
        let mut complete = false;
        for _ in 0..64 {
            for denied in [RetainedCloneGrant { maximum_copy_bytes: 0, ..authority.retirement }, RetainedCloneGrant { maximum_items: 0, ..authority.retirement }] {
                let (step, heap) = observe_heap_allocations_on_this_thread(|| PreparedAtlasPages::close_abandoned_step(denied));
                assert!(matches!(step, CloseStep::Pending { progress } if progress == Default::default()));
                assert_eq!(heap, HeapAllocationObservation::default());
                assert_eq!(PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
            }
            let quoted = unsafe { &*original }.original_close_progress();
            let (step, heap) = observe_heap_allocations_on_this_thread(|| PreparedAtlasPages::close_abandoned_step(authority.retirement));
            assert_eq!(step.progress(), quoted);
            assert!(step.progress().fits(authority.retirement));
            assert_eq!((quoted.retained_capacity_bytes, quoted.released_bytes), (heap.requested_bytes, heap.released_bytes));
            assert!(!heap.overflowed);
            release += heap.released_bytes;
            if PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire).is_null() {
                complete = matches!(PreparedAtlasPages::close_abandoned_step(authority.retirement), CloseStep::Complete { .. });
                break;
            }
        }
        assert!(complete);
        assert_eq!(release, expected);
        assert_eq!(PREPARED_ATLAS_ABANDONMENT_STATE[slot].load(Ordering::Acquire), 0);
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }
}
