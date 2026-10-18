//! 🧯️ Raw allocation close oracle of the retained command job: the testing function that closes a raw allocation through the original close ladder. See [`super`].

use super::*;

#[doc(hidden)]
#[cfg(any(test, feature = "artifact-app-testing"))]
pub fn test_raw_allocation_close<A: ArtifactApp>() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🚪️raw-allocation-close.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut raw = Vec::with_capacity(case["capacity"].as_u64().unwrap() as usize);
        raw.resize(case["initializedBytes"].as_u64().unwrap() as usize, 42);
        let mut job = ArtifactRetainedCommandJob::<A> {
            command: None,
            command_received:false,
            snapshot: None,
            snapshot_read: std::cell::RefCell::new(None),
            config: None,
            history: None,
            interaction_state: None,
            interaction_hover: None,
            context: None,
            operation: None,
            completion: None,
            command_id: |_| "fixture",
            maximum_raw_bytes: raw.capacity(),
            maximum_work_items: 1,
            work: None,
            checkpoint_input: None,
            checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_target: [MaybeUninit::uninit(); ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_capture: None,
            publication: RetainedJobPublication::new(),
            fault_publication: RetainedFaultPublication::new(),
            pending_publication: None,
            publication_delivered: false,
            fault: None,
            fault_retirement: None,
            decode_error: None,
            checkpoint_byte_len: 0,
            checkpoint_page_cursor: 0,
            checkpoint_page_offset: 0,
            raw_input: None,
            raw,
            admission_refusal:None,
            raw_page_cursor: 0,
            raw_page_offset: 0,
            emit: None,
            download:None,
            download_retirement:None,
            completion_retirement:None,
            input_retirement:None,
            ephemeral: None,
            phase: ArtifactRetainedCommandPhase::Complete,
            checkpoint_pending: false,
            work_progress: 0,
            closing: false,
        };
        job.begin_close();
        let capacity=job.raw.capacity();
        let pointer=job.raw.as_ptr();
        let expected:Vec<u8>=serde_json::from_str(&serde_json::to_string(&vec![42u8;case["initializedBytes"].as_u64().unwrap() as usize]).unwrap()).unwrap();
        assert_eq!(job.raw,expected);
        assert_eq!(job.next_close_copy_byte_demand().unwrap(),0);
        let unfunded=RetainedCloneGrant{maximum_items:0,maximum_depth:1,..Default::default()};
        assert!(matches!(job.close_step(unfunded),InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress} if progress==RetainedCloneProgress::default()));
        assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.capacity(),capacity);
        let mut copied=0;let mut released=0;
        for _ in 0..16 {
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:0,maximum_release_bytes:job.next_close_release_byte_demand().unwrap(),maximum_depth:job.next_close_depth_demand().unwrap()};
            let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant));
            let progress=match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,_=>panic!("funded original raw allocation must complete")};
            assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes);
            assert!(progress.copied_items<=grant.maximum_items&&progress.copied_bytes<=grant.maximum_copy_bytes&&progress.released_bytes<=grant.maximum_release_bytes);
            copied+=progress.copied_bytes;released+=progress.released_bytes;
            if job.terminal_is_empty(){break;}
        }
        assert!(job.terminal_is_empty());
        assert_eq!(copied,case["expectedCopiedBytes"].as_u64().unwrap() as usize);
        assert_eq!(released,case["expectedReleasedBytes"].as_u64().unwrap() as usize);
        assert_eq!(released,capacity);
    }
}
