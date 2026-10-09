use super::{ClipboardIoJob, ClipboardIoOperation};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Step, RetainedCloneGrant, RetainedCloneProgress};
use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;

thread_local! {
    static OBSERVATION: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
}

struct PhysicalAllocator;

unsafe impl GlobalAlloc for PhysicalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { semio_framework_trace::HeapWitness.alloc(layout) };
        if !pointer.is_null() {
            let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births + layout.size(), frees))) });
        }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births, frees + layout.size()))) });
        unsafe { semio_framework_trace::HeapWitness.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: PhysicalAllocator = PhysicalAllocator;

pub(crate) fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    OBSERVATION.with(|state| { assert!(state.get().is_none()); state.set(Some((0, 0))); });
    let result = operation();
    let (births, frees) = OBSERVATION.with(|state| state.replace(None).unwrap());
    (result, births, frees)
}

fn grant(copy: usize, release: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: release, maximum_depth: 1 }
}

#[test]
fn original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for (index, content) in fixture["texts"].as_array().unwrap().iter().enumerate() {
        let text = content.as_str().unwrap();
        let oracle: String = serde_json::from_value(content.clone()).unwrap();
        assert_eq!(oracle.as_bytes(), text.as_bytes());
        for copy in fixture["copyGrants"].as_array().unwrap() {
            for cancel in fixture["cancelTurns"].as_array().unwrap() {
                let mut original = String::with_capacity(fixture["capacities"][index].as_u64().unwrap() as usize);
                original.push_str(text);
                let pointer = original.as_ptr();
                let capacity = original.capacity();
                let (mut job, births, frees) = measured(|| ClipboardIoJob::write(original));
                assert_eq!((births, frees), (0, 0));
                let copy = copy.as_u64().unwrap() as usize;
                for _ in 0..cancel.as_u64().unwrap() {
                    let (result, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
                    assert!(matches!(result, Step::Blocked));
                    assert_eq!((births, frees), (0, 0));
                }
                job.begin_close();
                assert_eq!(job.next_close_copy_byte_demand().unwrap(), 0);
                assert_eq!(job.next_close_capacity_byte_demand(copy).unwrap(), 0);
                assert_eq!(job.next_close_release_byte_demand().unwrap(), capacity);
                let (refused, births, frees) = measured(|| job.close_step(grant(copy, capacity - 1)));
                assert!(matches!(refused, Step::Pending { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                match job.operation.as_ref().unwrap() {
                    ClipboardIoOperation::Write(retained) => { assert_eq!(retained.as_ptr(), pointer); assert_eq!(retained, &oracle); },
                    ClipboardIoOperation::Read => panic!("write ownership changed"),
                }
                let (terminal, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
                assert!(matches!(terminal, Step::Complete { progress } if progress.copied_items == 1 && progress.copied_bytes == 0 && progress.retained_capacity_bytes == births && progress.released_bytes == frees));
                assert_eq!((births, frees), (0, capacity));
                assert!(job.terminal_is_empty());
                let (repeat, births, frees) = measured(|| job.close_step(grant(0, 0)));
                assert!(matches!(repeat, Step::Complete { progress } if progress == RetainedCloneProgress::default()));
                assert_eq!((births, frees), (0, 0));
                let (_, births, frees) = measured(|| drop(job));
                assert_eq!((births, frees), (0, 0));
            }
        }
    }
}
