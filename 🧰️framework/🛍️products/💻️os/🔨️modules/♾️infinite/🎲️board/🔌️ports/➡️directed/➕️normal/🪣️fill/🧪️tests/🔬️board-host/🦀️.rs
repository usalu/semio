//! 🧪️ normal-fill tests (🔬️board-host): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_normal_fill::{BOARD_FILL_COMMIT_BYTES, BOARD_FILL_COMMIT_PAGE_COUNT, BOARD_FILL_KIND_HANDLE_CAPACITY, BOARD_FILL_PAGE_ITEMS, BOARD_FILL_PLACEMENT_CAPACITY, BOARD_FILL_SOURCE_CAPACITY, BoardFillCheckpoint, BoardFillCommitCandidate, BoardFillCommitEncoder, BoardFillCommitPlacement, BoardFillFixedPages, BoardFillJob, BoardFillJobState, BoardFillPlacement, BoardFillSnapshot, BoardFillSnapshotCapture, BoardFillSnapshotIngress, BoardFillSource, BoardFillVirtualHandle, NEUTRAL_BOARD_NAMING};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

#[cfg(test)]
#[allow(non_snake_case)]
fn BOARD_HOST_SOURCE() -> &'static str {
    fixture_text!("../../🦀️.rs")
}

#[cfg(test)]
    #[test]
    fn board_fill_descriptor_backings_are_exact_heap_owners_with_bounded_transfer_frames() {
        type SourcePages = BoardFillFixedPages<BoardFillSource, { (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        type VirtualHandlePages = BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        let sources = SourcePages::try_new().expect("source descriptor owner");
        let virtual_handles = VirtualHandlePages::try_new().expect("virtual-handle descriptor owner");
        assert_eq!(sources.pages.len(), (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(virtual_handles.pages.len(), (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(size_of::<SourcePages>(), 4 * size_of::<usize>());
        assert_eq!(size_of::<VirtualHandlePages>(), 4 * size_of::<usize>());
        assert!(size_of::<BoardFillSnapshot>() <= 1_024);
        assert!(size_of::<BoardFillSnapshotCapture>() <= 32 * 1_024);
        assert!(size_of::<BoardFillSnapshotIngress>() <= 32 * 1_024);
        assert!(size_of::<BoardFillPlacement>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitPlacement>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitCandidate>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCommitEncoder>() <= 32 * 1_024);
        assert!(size_of::<BoardFillJobState>() <= 32 * 1_024);
        assert!(size_of::<BoardFillCheckpoint>() <= 32 * 1_024);
        assert!(size_of::<BoardFillJob>() <= 32 * 1_024);
        assert!(size_of::<Result<BoardFillJob, BoardFillCheckpoint>>() <= 32 * 1_024);
        assert!(size_of::<semio_framework_job_worker::WorkerJobOutcome<BoardFillJob>>() <= 32 * 1_024);
        assert!(size_of::<semio_framework_job_worker::MountedWorkerJobSession<BoardFillJob>>() <= 32 * 1_024);
        assert!(!sources.terminal_is_empty());
        assert!(!virtual_handles.terminal_is_empty());
        assert!(BoardFillFixedPages::<u8, { usize::MAX }>::try_new().is_err());
    }

#[cfg(test)]
    #[test]
    fn board_fill_descriptor_retirement_needs_original_release_and_depth_authority() {
        use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
        let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️board-host-standalone/🧫️descriptor.json")).unwrap();
        let mut pages=BoardFillFixedPages::<u64,4>::new();
        let identity=pages.pages.as_ptr();
        let bytes=std::mem::size_of_val(pages.pages.as_ref());
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:bytes,maximum_depth:1};
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}] {
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(denied).unwrap());
            assert_eq!(step.progress(),RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(pages.pages.as_ptr(),identity);
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(grant).unwrap());
        assert_eq!(step.progress(),RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()});
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
        assert!(pages.terminal_is_empty());
        let reference=vec![0_u8;bytes].into_boxed_slice();
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(reference));
        assert_eq!(heap.released_bytes,step.progress().released_bytes);
        assert_eq!(serde_json::json!({"copiedItems":step.progress().copied_items,"birthBytes":0,"releaseMatchesOriginal":heap.released_bytes==bytes,"terminal":pages.terminal_is_empty()}),law["accepted"]);
    }

#[cfg(test)]
    fn fill_full_commit_candidate_contract(source: &str) -> bool {
        let Some(start) = source.find("pub struct BoardFillResult") else { return false };
        let Some(end) = source[start..].find("/// 🧵️ Persistent worker-owned fill search") else { return false };
        let codec = &source[start..start + end];
        let Some(job_start) = source.find("fn complete(&mut self, context: &mut semio_framework_job::StepContext") else { return false };
        let Some(job_end) = source[job_start..].find("fn fault_outcome") else { return false };
        let job = &source[job_start..job_start + job_end];
        codec.contains("pub struct BoardFillCommitPlacement")
            && codec.contains("pub edge_kind: BoardFillText")
            && codec.contains("pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY]")
            && codec.contains("pub placement: Option<BoardFillCommitPlacement>")
            && codec.contains("struct BoardFillCommitPayload")
            && codec.contains("BoardFillCommitPayload::new(&candidate.output)")
            && codec.contains("payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT")
            && codec.contains("payload.page(page_index)")
            && codec.contains("text_byte: usize")
            && codec.contains("fn step_text")
            && codec.contains("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte]")
            && !codec.contains("String")
            && !codec.contains("Vec<")
            && !codec.contains("BTreeMap")
            && !codec.contains("single_page()")
            && !codec.contains("const BOARD_FILL_COMMIT_BYTES: usize = 13")
            && !codec.contains("result.encode")
            && job.contains("start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)")
            && job.contains("if end < BOARD_FILL_COMMIT_BYTES")
            && job.contains("encoder.output_cursor = end")
    }

#[cfg(test)]
    #[test]
    fn fill_full_commit_candidate_mutations_are_rejected() {
        let source = BOARD_HOST_SOURCE();
        assert_eq!(BOARD_FILL_COMMIT_BYTES, 10_406);
        assert_eq!(BOARD_FILL_COMMIT_PAGE_COUNT, 1);
        assert!(BOARD_FILL_COMMIT_BYTES <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES * BOARD_FILL_COMMIT_PAGE_COUNT);
        assert!(fill_full_commit_candidate_contract(source));
        let dynamic = source.replacen("pub struct BoardFillCommitPlacement {\n        pub node_id: BoardFillText,", "pub struct BoardFillCommitPlacement {\n        pub dynamic: String,\n        pub node_id: BoardFillText,", 1);
        assert!(!fill_full_commit_candidate_contract(&dynamic));
        let summary = source.replacen("const BOARD_FILL_COMMIT_BYTES: usize = BOARD_FILL_COMMIT_HANDLE_OFFSET + BOARD_FILL_KIND_HANDLE_CAPACITY * BOARD_FILL_COMMIT_HANDLE_BYTES;", "const BOARD_FILL_COMMIT_BYTES: usize = 13;", 1);
        assert!(!fill_full_commit_candidate_contract(&summary));
    }

#[cfg(test)]
    /// ♻️ Every fill close turn quotes its exact release demand, yields without mutation below any demand axis,
    /// and answers with a receipt that fits the unchanged grant and sums to the owner's physical page backing.
    #[test]
    fn fill_close_turns_quote_exact_demands_and_fit_their_unchanged_grants() {
        use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        let mut ingress = BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING);
        for index in 0..BOARD_FILL_PAGE_ITEMS * 2 + 1 {
            ingress.begin_node().expect("node begins");
            ingress.push_node_id_byte(b'a' + index as u8).expect("node id byte");
            ingress.publish_node().expect("node publishes");
        }
        ingress.begin_node().expect("open node stays unpublished");
        let backing: usize = ingress.snapshot.as_ref().expect("snapshot").nodes.pages.iter().flatten().map(|page| page.backing_bytes).sum();
        assert!(backing > 0);
        let mut released = 0usize;
        for turn in 0.. {
            assert!(turn < 64, "ingress close reaches terminal-empty");
            let release = ingress.next_close_release_byte_demand();
            let exact = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: release, maximum_depth: 1, ..Default::default() };
            let under = [Some(RetainedCloneGrant { maximum_items: 0, ..exact }), Some(RetainedCloneGrant { maximum_depth: 0, ..exact }), (release > 0).then(|| RetainedCloneGrant { maximum_release_bytes: release - 1, ..exact })];
            for grant in under.into_iter().flatten() {
                assert_eq!(ingress.close_step(grant), InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(ingress.next_close_release_byte_demand(), release, "a yield never mutates the owner");
            }
            let (InteractiveJobCloseStep::Pending { progress } | InteractiveJobCloseStep::Complete { progress }) = ingress.close_step(exact) else { panic!("ingress close was refused or blocked") };
            assert!(progress.fits(exact));
            released += progress.released_bytes;
            if ingress.terminal_is_empty() {
                break;
            }
        }
        assert_eq!(released, backing, "the receipts account for every retired page backing");
        let mut seed = BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING);
        seed.begin_node().expect("seed node begins");
        seed.push_node_id_byte(b'n').expect("seed node id byte");
        seed.publish_node().expect("seed node publishes");
        let snapshot = seed.take_snapshot().expect("seed snapshot is complete");
        let mut job = BoardFillJob::with_operation(snapshot, 1, semio_framework_job::Operation::new(semio_framework_job::OperationId(1), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 1));
        for turn in 0.. {
            assert!(turn < 64, "job close reaches terminal-empty");
            let copy = job.next_close_copy_byte_demand().expect("job copy demand");
            let exact = RetainedCloneGrant {
                maximum_items: 1,
                maximum_copy_bytes: copy,
                maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).expect("job capacity demand"),
                maximum_release_bytes: job.next_close_release_byte_demand().expect("job release demand"),
                maximum_depth: job.next_close_depth_demand().expect("job depth demand"),
            };
            let under = [Some(RetainedCloneGrant { maximum_items: 0, ..exact }), Some(RetainedCloneGrant { maximum_depth: 0, ..exact }), (exact.maximum_release_bytes > 0).then(|| RetainedCloneGrant { maximum_release_bytes: exact.maximum_release_bytes - 1, ..exact })];
            for grant in under.into_iter().flatten() {
                assert_eq!(job.close_step(grant), InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(job.next_close_release_byte_demand().expect("job release demand"), exact.maximum_release_bytes, "a yield never mutates the job");
            }
            match job.close_step(exact) {
                InteractiveJobCloseStep::Pending { progress } => assert!(progress.fits(exact)),
                InteractiveJobCloseStep::Complete { progress } => {
                    assert!(progress.fits(exact));
                    break;
                }
                other => panic!("job close ended in {other:?}"),
            }
        }
        assert!(job.terminal_is_empty());
    }

#[cfg(test)]
#[test]
fn empty_fill_capture_and_ingress_keep_every_original_descriptor_until_admitted() {
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
    let(mut capture,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotCapture::new(0.0, &NEUTRAL_BOARD_NAMING));
    let mut released=0;
    for turn in 0..16{
        let release=capture.next_close_release_byte_demand();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:1};
        if release!=0{
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(RetainedCloneGrant{maximum_release_bytes:release-1,..grant}));
            assert_eq!(step.progress(),RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(capture.next_close_release_byte_demand(),release);
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(grant));
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,step.progress().released_bytes));
        released+=heap.released_bytes;
        if capture.terminal_is_empty(){break;}
        assert!(turn<15);
    }
    assert!(capture.terminal_is_empty());
    assert_eq!(released,birth.requested_bytes);
    let(mut ingress,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotIngress::new(0.0, &NEUTRAL_BOARD_NAMING));
    let mut released=0;
    for turn in 0..16{
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:ingress.next_close_release_byte_demand(),maximum_depth:1};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant));
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,step.progress().released_bytes));
        released+=heap.released_bytes;
        if ingress.terminal_is_empty(){break;}
        assert!(turn<15);
    }
    assert!(ingress.terminal_is_empty());
    assert_eq!(released,birth.requested_bytes);
}
