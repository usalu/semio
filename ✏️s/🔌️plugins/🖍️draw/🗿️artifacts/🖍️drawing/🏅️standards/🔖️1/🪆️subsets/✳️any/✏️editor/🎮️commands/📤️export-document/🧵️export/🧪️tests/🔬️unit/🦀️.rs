//! 🧪️ Original export sources survive interruption and every denied retirement currency.
use semio_framework_diagnostic::Fault;
use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkPoll, ValueError};
use semio_os_kernel::io::io_mechanism::{OwnedSerializerJob, OwnedSerializerOutput, OwnedSerializerStep};
use semio_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_s_artifact_draw_drawing_editor::export_job::testkit;
use semio_s_artifact_draw_drawing_model::{DRAWING_DOCUMENT_SCHEMA, snapshot::DrawingSnapshot};
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller, OriginalRetirementPump};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{ErasedSnapshotRetirement, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetirementDemand};

type Store = store::ArtifactStore<DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation>;

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn effect_turn<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
    turn.with_effect(run).unwrap().unwrap().0
}

fn progress(poll: OriginalRetirementPoll) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap().progress(),
        RetainedWorkPoll::Pending { .. } => Default::default(),
    }
}

fn pump_returns(issuer: &mut Store, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>) {
    if active.is_none() && issuer.returned_snapshot_read_count() == 0 { return; }
    let body = active.as_ref().map_or(0, |owner| {
        owner.next_copy_byte_demand().unwrap().max(owner.next_release_byte_demand().unwrap())
    });
    let demand = match active.as_ref() {
        Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body),
        None => issuer.returned_snapshot_read_retirement_demand(),
    }.unwrap();
    let selected = grant(demand);
    let (receipt, physical) = observe(|| effect_turn(selected, |effect| {
        if active.is_some() {
            return progress(store::os_store::snapshot_read::artifact_retirement_box_close_step(active, body, effect));
        }
        let (owner, receipt) = issuer.take_returned_snapshot_read_retirement(effect).unwrap();
        *active = owner;
        receipt
    }));
    assert!(receipt.fits(selected));
    assert!(!physical.overflowed);
    assert_eq!((physical.requested_bytes, physical.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
}

#[semio_framework_async_macros::async_test]
async fn original_export_read_returns_under_exact_physical_grants() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &corpus["readReturn"]["exportConsumer"];
    assert_eq!(law["identity"], "original-read");

    let transport: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️transport/🔣️.json")).unwrap();
    let header = &transport["header"];
    let id = header["documentId"].as_str().unwrap();
    let document = DrawingSnapshot { id: id.into(), ..Default::default() };
    let envelope = store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation>(
        DRAWING_DOCUMENT_SCHEMA, id, document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    );
    let mut issuer = Store::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    issuer.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners().unwrap());
    let read = issuer.snapshot_read().unwrap();
    let original = read.get() as *const DrawingSnapshot;
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x44524558), Generation(issuer.generation()), stock, || Some(0), u64::MAX, 1 << 24);
    let mut work = testkit::work_with_read(read);
    let payload = semio_s_artifact_draw_drawing_editor_commands::export_document::ExportDocument {
        format: header["format"].as_str().unwrap().into(),
        width: Some(header["width"].as_u64().unwrap().try_into().unwrap()),
        height: Some(header["height"].as_u64().unwrap().try_into().unwrap()),
        transparent: Some(header["transparent"].as_bool().unwrap()),
    };
    let selected = grant(testkit::header_demands(&work, &payload).unwrap());
    let generation = issuer.generation();
    let revision = issuer.content_revision();
    for denied in [
        RetainedCloneGrant { maximum_items: 0, ..selected },
        RetainedCloneGrant { maximum_copy_bytes: selected.maximum_copy_bytes - 1, ..selected },
        RetainedCloneGrant { maximum_capacity_bytes: selected.maximum_capacity_bytes - 1, ..selected },
        RetainedCloneGrant { maximum_depth: 0, ..selected },
    ] {
        caller.turn(selected, |cx| cx.with_retained_loan(denied, |cx| {
            let (result, physical) = observe(|| testkit::prepare_header(&mut work, &payload, generation, revision, cx));
            assert!(!result?);
            assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
            assert_eq!(testkit::state(&work).read.unwrap().get() as *const DrawingSnapshot, original);
            Ok(())
        })).unwrap();
    }
    caller.turn(selected, |cx| {
        let before = cx.retained_progress();
        let (result, physical) = observe(|| testkit::prepare_header(&mut work, &payload, generation, revision, cx));
        assert!(result?);
        let after = cx.retained_progress();
        assert_eq!(after.copied_bytes - before.copied_bytes, selected.maximum_copy_bytes);
        assert_eq!((physical.requested_bytes, physical.released_bytes), (selected.maximum_capacity_bytes, 0));
        assert_eq!(after.retained_capacity_bytes - before.retained_capacity_bytes, selected.maximum_capacity_bytes);
        let state = testkit::state(&work);
        let request = state.request.unwrap();
        assert_eq!(request.source.get::<DrawingSnapshot>().unwrap() as *const DrawingSnapshot, original);
        assert_eq!(request.route.1.artifact_kind, header["artifactKind"].as_str().unwrap());
        assert_eq!(state.labels.unwrap(), (header["filename"].as_str().unwrap(), header["mime"].as_str().unwrap()));
        assert_eq!(request.options.as_object().unwrap().len(), 3);
        Ok(())
    }).unwrap();
    work.begin_close();
    let mut returned = None;

    for _ in 0..100_000 {
        pump_returns(&mut issuer, &mut returned);
        if work.terminal_is_empty() { break; }
        if let Some(read) = testkit::state(&work).read { assert_eq!(read.get() as *const DrawingSnapshot, original); }
        let selected = grant(work.close_demands(4096).unwrap());
        for axis in law["denied"].as_array().unwrap() {
            let mut denied = selected;
            let budget = match axis.as_str().unwrap() {
                "items" => &mut denied.maximum_items,
                "copy" => &mut denied.maximum_copy_bytes,
                "capacity" => &mut denied.maximum_capacity_bytes,
                "release" => &mut denied.maximum_release_bytes,
                "depth" => &mut denied.maximum_depth,
                _ => unreachable!(),
            };
            if *budget == 0 { continue; }
            *budget -= 1;
            caller.turn(selected, |cx| cx.with_original_retirement(|cx| {
                let (result, physical) = observe(|| cx.with_retained_loan(denied, |cx| work.close_step(cx)));
                assert!(!physical.overflowed);
                assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
                match result {
                    Ok(step) => assert_eq!(step.progress(), Default::default()),
                    Err(error) => assert_eq!(error.effects, Default::default()),
                }
                Ok(())
            })).unwrap();
            if let Some(read) = testkit::state(&work).read { assert_eq!(read.get() as *const DrawingSnapshot, original); }
        }
        caller.turn(selected, |cx| cx.with_original_retirement(|cx| {
            let (result, physical) = observe(|| work.close_step(cx));
            let step = result?;
            assert!(matches!(step, InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. }));
            let receipt = step.progress();
            assert!(receipt.fits(selected));
            assert!(!physical.overflowed);
            assert_eq!((physical.requested_bytes, physical.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            Ok(())
        })).unwrap();
    }
    assert!(work.terminal_is_empty());
    for _ in 0..100_000 {
        if returned.is_none() && issuer.returned_snapshot_read_count() == 0 { break; }
        pump_returns(&mut issuer, &mut returned);
    }
    assert!(returned.is_none() && issuer.returned_snapshot_read_count() == 0);
    let (_, physical) = observe(|| drop(work));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    caller.retire_exact();

    for _ in 0..100_000 {
        if issuer.close_owned_terminal_is_empty() { break; }
        let selected = grant(issuer.close_owned_demands(4096).unwrap());
        effect_turn(selected, |effect| progress(issuer.close_owned_step(effect)));
    }
    assert!(issuer.close_owned_terminal_is_empty());
}

#[test]
fn export_terminal_handoffs_charge_the_original_turn() {
    use semio_framework_job::{InteractiveJob, InteractiveJobStep, JobOutcomeDescriptor, JobOutcomeView, OriginalJobReturnClaim, StepContext};
    use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
    use semio_os_kernel::io::io_mechanism::{OwnedSerializerFaultHandoff, OwnedSerializerOutputHandoff};

    struct TerminalSerializer {
        output: Option<OwnedSerializerOutput>,
        fault: Option<Fault>,
    }

    impl InteractiveJob for TerminalSerializer {
        fn step<'a>(&'a mut self, _: &mut StepContext<'_>, _: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> { unreachable!() }
        fn borrow_outcome<'a>(&'a self, _: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { unreachable!() }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, _: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { unreachable!() }
        fn terminal_is_empty(&self) -> bool { self.output.is_none() && self.fault.is_none() }
    }

    impl OwnedSerializerJob for TerminalSerializer {
        fn advance_serialization(&mut self, _: &mut StepContext<'_>) -> OwnedSerializerStep { unreachable!() }
        fn source_authority_matches(&self, _: u64, _: [u8; 32]) -> bool { true }
        fn output(&self) -> Option<&OwnedSerializerOutput> { self.output.as_ref() }
        fn take_output(&mut self, grant: RetainedCloneGrant) -> Result<Option<OwnedSerializerOutputHandoff>, ValueError> {
            let progress = RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 }.admit(grant)?;
            Ok(self.output.take().map(|output| OwnedSerializerOutputHandoff { output, progress }))
        }
        fn fault(&self) -> Option<&Fault> { self.fault.as_ref() }
        fn take_fault(&mut self, grant: RetainedCloneGrant) -> Result<Option<OwnedSerializerFaultHandoff>, ValueError> {
            let progress = RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 }.admit(grant)?;
            Ok(self.fault.take().map(|fault| OwnedSerializerFaultHandoff { fault, progress }))
        }
        fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
    }

    let exact = RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() };
    let receipt = RetainedCloneProgress { copied_items: 1, ..Default::default() };
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x44524559), Generation(1), stock, || Some(0), u64::MAX, 1 << 24);
    for phase in ["handoff-output", "handoff-fault"] {
        let fixture = TerminalSerializer {
            output: (phase == "handoff-output").then(OwnedSerializerOutput::empty),
            fault: (phase == "handoff-fault").then(|| Fault::from("Drawing fixture serializer fault")),
        };
        let mut work = testkit::work_with_serializer(Box::new(fixture), phase == "handoff-fault");
        for denied in [RetainedCloneGrant { maximum_items: 0, ..exact }, RetainedCloneGrant { maximum_depth: 0, ..exact }] {
            assert!(matches!(caller.turn(exact, |cx| cx.with_retained_loan(denied, |cx| {
                let before = cx.fuel_remaining();
                let step = testkit::handoff(&mut work, cx)?;
                assert_eq!(cx.fuel_remaining(), before);
                Ok(matches!(step, ArtifactCommandWorkStep::Pending { .. }))
            })), Ok(true)));
            assert_eq!(caller.receipt(), Default::default());
            let job = testkit::state(&work).job.unwrap();
            assert_eq!(job.output().is_some(), phase == "handoff-output");
            assert_eq!(job.fault().is_some(), phase == "handoff-fault");
        }
        caller.turn(exact, |cx| {
            let before = cx.fuel_remaining();
            let step = testkit::handoff(&mut work, cx);
            if phase == "handoff-output" {
                assert!(matches!(step, Ok(ArtifactCommandWorkStep::Progress { stage: "output", .. })));
                assert!(testkit::state(&work).raw.is_some());
            } else {
                assert!(step.is_err());
            }
            assert_eq!(cx.fuel_remaining(), before - 1);
            Ok(())
        }).unwrap();
        assert_eq!(caller.receipt(), receipt);
        let job = testkit::state(&work).job.unwrap();
        assert!(job.output().is_none() && job.fault().is_none());
        work.begin_close();
        for _ in 0..100 {
            if work.terminal_is_empty() { break; }
            let selected = grant(work.close_demands(4096).unwrap());
            caller.turn(selected, |cx| cx.with_original_retirement(|cx| work.close_step(cx))).unwrap();
        }
        assert!(work.terminal_is_empty());
    }
    caller.retire_exact();
}

#[test]
fn export_retires_the_original_segmented_transport() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️transport/🔣️.json")).unwrap();
    assert_eq!(law["inputPageBytes"].as_u64().unwrap() as usize, semio_plugin::app::ArtifactOutputChunks::CHUNK_BYTES / 4 * 3);
    let repetitions = law["fullWordCount"].as_u64().unwrap() as usize;
    let full = law["fullWord"].as_str().unwrap().repeat(repetitions).into_bytes();
    let tail = law["tail"].as_str().unwrap().as_bytes();
    let full_encoded = law["encodedFullWord"].as_str().unwrap().repeat(repetitions);
    let tail_encoded = law["encodedTail"].as_str().unwrap().as_bytes();
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128 };
    let turn = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 128 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x4452455a), Generation(1), stock, || Some(0), u64::MAX, 1 << 24);
    for source in law["sources"].as_array().unwrap() {
        for completion in law["completion"].as_array().unwrap() {
            let mut raw = OwnedSerializerOutput::empty();
            match source.as_str().unwrap() {
                "paged" => raw.pages = [full.clone(), tail.to_vec()].into_iter().collect(),
                "contiguous" => { raw.contiguous.extend_from_slice(&full); raw.contiguous.extend_from_slice(tail); }
                _ => unreachable!(),
            }
            let mut work = testkit::work_with_output(raw);
            let mut staged = Vec::new();
            let mut reserved_queue_page = false;
            for _ in 0..16 {
                if testkit::state(&work).stage == "publish" { break; }
                let pending = testkit::state(&work).pending_page.map(|page| page.as_ptr());
                let demand = testkit::output_demands(&work).unwrap();
                let denied = RetainedCloneGrant { maximum_depth: 0, ..grant(demand) };
                caller.turn(turn, |cx| cx.with_retained_loan(denied, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = observe(|| testkit::advance_output(&mut work, cx));
                    result?;
                    assert_eq!(testkit::state(&work).pending_page.map(|page| page.as_ptr()), pending);
                    assert_eq!(cx.retained_progress(), before);
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    Ok(())
                })).unwrap();
                caller.turn(turn, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = observe(|| testkit::advance_output(&mut work, cx));
                    result?;
                    let after = cx.retained_progress();
                    let born = after.retained_capacity_bytes - before.retained_capacity_bytes;
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (born, after.released_bytes - before.released_bytes));
                    assert_eq!(after.copied_items - before.copied_items, 1);
                    let current = testkit::state(&work).pending_page.map(|page| page.as_ptr());
                    if pending.is_none() {
                        if let Some(pointer) = current { staged.push(pointer); }
                    } else if current == pending && born > 0 {
                        reserved_queue_page = true;
                    }
                    Ok(())
                }).unwrap();
                if completion == "cancel-with-staged-page" && !staged.is_empty() { break; }
            }
            if completion == "seal" {
                assert!(reserved_queue_page);
                assert_eq!(testkit::state(&work).stage, "publish");
                let chunks = testkit::state(&work).chunks.unwrap();
                assert!(chunks.is_sealed());
                assert_eq!(chunks.chunks_remaining(), 2);
                for (index, expected) in [full_encoded.as_bytes(), tail_encoded].into_iter().enumerate() {
                    let chunk = chunks.take_chunk().unwrap().unwrap();
                    assert_eq!(chunk.as_ptr(), staged[index]);
                    assert_eq!(chunk, expected);
                }
                assert!(chunks.take_chunk().unwrap().is_none());
            } else {
                assert_eq!(staged.len(), 1);
                assert!(testkit::state(&work).pending_page.is_some());
            }
            work.begin_close();
            for _ in 0..100_000 {
                if work.terminal_is_empty() { break; }
                let selected = grant(work.close_demands(4096).unwrap());
                caller.turn(selected, |cx| cx.with_original_retirement(|cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = observe(|| work.close_step(cx));
                    result?;
                    let after = cx.retained_progress();
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                    Ok(())
                })).unwrap();
            }
            assert!(work.terminal_is_empty());
        }
    }
    caller.retire_exact();
}
