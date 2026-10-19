use semio_framework_os_infinite_board_normal_fill::{BoardFillNodeCapture, BoardFillNodeSnapshot, BoardFillText, BoardFillCaptureStep};
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
        let mut original = BoardFillOriginalFixture::new();
        let mut ingress = original.ingress();
        for index in 0..BOARD_FILL_PAGE_ITEMS * 2 + 1 {
            ingress.begin_node().expect("node begins");
            ingress.push_node_id_byte(b'a' + index as u8).expect("node id byte");
            ingress.publish_node().expect("node publishes");
        }
        ingress.begin_node().expect("open node stays unpublished");
        let source = ingress.snapshot.as_ref().expect("snapshot");
        let backing: usize = source.nodes.pages.iter().flatten().map(|page| page.backing_bytes).sum::<usize>() + size_of_val(source.nodes.pages.as_ref()) + size_of_val(source.handles.pages.as_ref()) + size_of_val(source.kinds.pages.as_ref()) + size_of_val(source.rules.pages.as_ref());
        assert!(backing > 0);
        let mut released = 0usize;
        for turn in 0.. {
            assert!(turn < 64, "ingress close reaches terminal-empty");
            let release = ingress.next_close_release_byte_demand();
            let exact = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: release, maximum_depth: 1, ..Default::default() };
            let under = [Some(RetainedCloneGrant { maximum_items: 0, ..exact }), Some(RetainedCloneGrant { maximum_depth: 0, ..exact }), (release > 0).then(|| RetainedCloneGrant { maximum_release_bytes: release - 1, ..exact })];
            for grant in under.into_iter().flatten() {
                assert_eq!(original.close_ingress(&mut ingress, grant), InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(ingress.next_close_release_byte_demand(), release, "a yield never mutates the owner");
            }
            let (InteractiveJobCloseStep::Pending { progress } | InteractiveJobCloseStep::Complete { progress }) = original.close_ingress(&mut ingress, exact) else { panic!("ingress close was refused or blocked") };
            assert!(progress.fits(exact));
            released += progress.released_bytes;
            if ingress.terminal_is_empty() {
                break;
            }
        }
        assert_eq!(released, backing, "the receipts account for every retired page backing");
        let mut seed = original.ingress();
        seed.begin_node().expect("seed node begins");
        seed.push_node_id_byte(b'n').expect("seed node id byte");
        seed.publish_node().expect("seed node publishes");
        let snapshot = seed.take_snapshot().expect("seed snapshot is complete");
        let mut job = original.job(snapshot, 1, semio_framework_job::Operation::new(semio_framework_job::OperationId(1), semio_framework_job::RevisionId(1), semio_framework_job::Generation(1), 1));
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
                assert_eq!(original.close_job(&mut job, grant), InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(job.next_close_release_byte_demand().expect("job release demand"), exact.maximum_release_bytes, "a yield never mutates the job");
            }
            match original.close_job(&mut job, exact) {
                InteractiveJobCloseStep::Pending { progress } => assert!(progress.fits(exact)),
                InteractiveJobCloseStep::Complete { progress } => {
                    assert!(progress.fits(exact));
                    break;
                }
                other => panic!("job close ended in {other:?}"),
            }
        }
        assert!(job.terminal_is_empty());
        while !seed.terminal_is_empty() { let grant = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: seed.next_close_release_byte_demand(), maximum_depth: 1, ..Default::default() }; original.close_ingress(&mut seed, grant); }
        original.finish();
    }

#[cfg(test)]
#[test]
fn empty_fill_capture_and_ingress_keep_every_original_descriptor_until_admitted() {
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
    let mut original = BoardFillOriginalFixture::new();
    let(mut capture,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.capture());
    let mut released=0;
    for turn in 0..16{
        let release=capture.next_close_release_byte_demand();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:1};
        if release!=0{
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close_capture(&mut capture, RetainedCloneGrant{maximum_release_bytes:release-1,..grant}));
            assert_eq!(step.progress(),RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(capture.next_close_release_byte_demand(),release);
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close_capture(&mut capture,grant));
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,step.progress().released_bytes));
        released+=heap.released_bytes;
        if capture.terminal_is_empty(){break;}
        assert!(turn<15);
    }
    assert!(capture.terminal_is_empty());
    assert_eq!(released,birth.requested_bytes);
    let(mut ingress,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.ingress());
    let mut released=0;
    for turn in 0..16{
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:ingress.next_close_release_byte_demand(),maximum_depth:1};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.close_ingress(&mut ingress,grant));
        assert!(step.progress().fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,step.progress().released_bytes));
        released+=heap.released_bytes;
        if ingress.terminal_is_empty(){break;}
        assert!(turn<15);
    }
    assert!(ingress.terminal_is_empty());
    assert_eq!(released,birth.requested_bytes);
    original.finish();
    eprintln!("[DEBUG] empty fill ingress and capture retain all descriptor custody until exact physical release");
}

#[cfg(test)]
struct BoardFillOriginalFixture { caller: semio_framework_job::OriginalJobFixtureCaller, decoder: semio_framework_job::OriginalJobDecodeOwner }
#[cfg(test)]
impl BoardFillOriginalFixture {
    fn stock() -> semio_framework_value::RetainedCloneGrant { semio_framework_value::RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 64 << 20, maximum_capacity_bytes: 64 << 20, maximum_release_bytes: 64 << 20, maximum_depth: 128 } }
    fn new() -> Self { Self { caller: semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(11), semio_framework_job::Generation(3), Self::stock(), || Some(0), u64::MAX, 100_000), decoder: semio_framework_job::OriginalJobDecodeOwner::empty() } }
    fn ingress(&mut self) -> BoardFillSnapshotIngress {
        let mut output = None;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| self.caller.turn_with_decoder(Self::stock(), &mut self.decoder, |cx| BoardFillSnapshotIngress::admit_original(0.0, &NEUTRAL_BOARD_NAMING, &mut output, cx)));
        assert!(result.unwrap()); let receipt = self.caller.receipt(); assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes),(heap.requested_bytes,heap.released_bytes)); output.unwrap()
    }
    fn capture(&mut self) -> BoardFillSnapshotCapture {
        let mut output = None;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| self.caller.turn_with_decoder(Self::stock(), &mut self.decoder, |cx| BoardFillSnapshotCapture::admit_original(0.0, &NEUTRAL_BOARD_NAMING, &mut output, cx)));
        assert!(result.unwrap()); let receipt = self.caller.receipt(); assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes),(heap.requested_bytes,heap.released_bytes)); output.unwrap()
    }
    fn job(&mut self, snapshot: BoardFillSnapshot, count: u32, semantic: semio_framework_job::Operation) -> BoardFillJob {
        let mut source = Some(snapshot); let mut output = None;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| self.caller.turn_with_decoder(Self::stock(), &mut self.decoder, |cx| BoardFillJob::admit_original(&mut source, count, semantic, &mut output, cx)));
        assert!(result.unwrap()); assert!(source.is_none()); let receipt = self.caller.receipt(); assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes),(heap.requested_bytes,heap.released_bytes)); output.unwrap()
    }
    fn close_job(&mut self, job: &mut BoardFillJob, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep { use semio_framework_job::InteractiveJob; self.caller.turn(grant, |cx| cx.with_original_retirement(|cx| job.close_step(cx))).unwrap() }
    fn close_ingress(&mut self, owner: &mut BoardFillSnapshotIngress, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let demand = semio_framework_value::RetirementDemand { release_bytes: owner.next_close_release_byte_demand(), depth: 1, ..Default::default() };
        self.caller.turn(grant, |cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(semio_framework_value::RetainedCloneStep::Progress(owner.close_step(grant).progress())))).unwrap()
    }
    fn close_capture(&mut self, owner: &mut BoardFillSnapshotCapture, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let demand = semio_framework_value::RetirementDemand { release_bytes: owner.next_close_release_byte_demand(), depth: 1, ..Default::default() };
        self.caller.turn(grant, |cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(semio_framework_value::RetainedCloneStep::Progress(owner.close_step(grant).progress())))).unwrap()
    }
    fn finish(mut self) {
        while !self.decoder.terminal_is_empty() { self.caller.turn(Self::stock(), |cx| { let step = self.decoder.close_step(cx.retained_grant())?; cx.consume_retained(step.progress())?; cx.consume_fuel(step.progress().copied_items as u64); Ok(()) }).unwrap(); }
        self.caller.retire(Self::stock());
    }
}

#[cfg(test)]
#[test]
fn board_fill_original_descriptors_keep_source_and_semantic_serial_separate() {
    use semio_framework_job::{InteractiveJob, Operation, OperationId, RevisionId, Generation};
    let law: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎟️admission/🔣️.json")).unwrap();
    let mut original = BoardFillOriginalFixture::new();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut denied = None;
    let demand = BoardFillSnapshotIngress::admission_demands().unwrap();
    let exact = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
    for axis in law["deniedAxes"].as_array().unwrap() {
        let mut grant = exact;
        match axis.as_str().unwrap() { "items" => grant.maximum_items = 0, "copy" => grant.maximum_copy_bytes -= 1, "capacity" => grant.maximum_capacity_bytes -= 1, "depth" => grant.maximum_depth = 0, _ => panic!("unknown original denial axis") }
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.caller.turn_with_decoder(grant, &mut decoder, |cx| BoardFillSnapshotIngress::admit_original(0.0, &NEUTRAL_BOARD_NAMING, &mut denied, cx)));
        assert!(!result.unwrap()); assert!(denied.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    let mut ingress = original.ingress();
    let snapshot = ingress.take_snapshot().expect("authentic original empty snapshot");
    let semantic = Operation::new(OperationId(law["semanticSerial"].as_u64().unwrap()), RevisionId(0), Generation(law["semanticGeneration"].as_u64().unwrap()), law["seed"].as_u64().unwrap());
    let mut job = original.job(snapshot, 0, semantic);
    assert_eq!(job.state.as_ref().unwrap().next_serial, semantic.operation.0);
    let mut foreign = BoardFillOriginalFixture::new();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| foreign.caller.turn(BoardFillOriginalFixture::stock(), |cx| job.close_step(cx)));
    assert!(result.is_err()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    while !job.terminal_is_empty() { let grant = original.caller.remaining_grant(); original.close_job(&mut job, grant); }
    while !ingress.terminal_is_empty() { let grant = original.caller.remaining_grant(); original.close_ingress(&mut ingress, grant); }
    while !decoder.terminal_is_empty() { original.caller.turn(BoardFillOriginalFixture::stock(), |cx| { let step = decoder.close_step(cx.retained_grant())?; cx.consume_retained(step.progress())?; cx.consume_fuel(step.progress().copied_items as u64); Ok(()) }).unwrap(); }
    foreign.finish(); original.finish();
}

#[cfg(test)]
#[test]
fn board_fill_capture_original_page_births_keep_pending_on_each_denied_axis() {
    let mut original = BoardFillOriginalFixture::new();
    let mut capture = original.capture();
    let scene = semio_framework_os_infinite_board_normal_scene::BoardSceneState::default();
    let style = semio_framework_os_infinite_board_normal_scene::BoardStyleState::default();
    let brush = semio_framework_os_infinite_board_normal_gestures::BoardBrushState::default();
    for index in 0..=BOARD_FILL_PAGE_ITEMS {
        capture.node = Some(BoardFillNodeCapture { id: BoardFillText::empty(), bounds: Some([0.0; 4]) });
        let demand = capture.next_step_demands(&style).unwrap();
        assert_eq!(demand.capacity_bytes, if index % BOARD_FILL_PAGE_ITEMS == 0 { BOARD_FILL_PAGE_ITEMS * size_of::<BoardFillNodeSnapshot>() } else { 0 });
        for axis in 0..4 {
            let mut denied = BoardFillOriginalFixture::stock();
            match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = demand.copy_bytes - 1, 2 if demand.capacity_bytes != 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, 2 => continue, _ => denied.maximum_depth = demand.depth - 1 }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.caller.turn_with_decoder(denied, &mut original.decoder, |cx| capture.step(&scene, &style, &brush, cx)));
            assert!(matches!(step.unwrap(), BoardFillCaptureStep::Pending));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(original.caller.receipt(), Default::default());
            assert!(capture.node.is_some());
            assert_eq!(capture.snapshot.as_ref().unwrap().nodes.len, index);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original.caller.turn_with_decoder(BoardFillOriginalFixture::stock(), &mut original.decoder, |cx| capture.step(&scene, &style, &brush, cx)));
        assert!(matches!(step.unwrap(), BoardFillCaptureStep::Pending));
        let receipt = original.caller.receipt();
        assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
        assert_eq!(capture.snapshot.as_ref().unwrap().nodes.len, index + 1);
        assert!(capture.node.is_none());
    }
    while !capture.terminal_is_empty() { original.close_capture(&mut capture, BoardFillOriginalFixture::stock()); }
    drop(capture);
    original.finish();
    eprintln!("[DEBUG] original Board capture page boundaries preserve pending source on all denied axes and reconcile actual allocator bytes once");
}
