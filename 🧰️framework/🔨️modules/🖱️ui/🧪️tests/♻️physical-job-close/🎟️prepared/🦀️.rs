use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use super::{atlas_fixture_authority, atlas_fixture_owner, atlas_fixture_page};
use super::measured;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Close, RetainedCloneGrant};

#[test]
fn prepared_original_owner_closes_through_the_actual_job_full_grant_contract() {
    let _guard = prepared_process_guard();
    let mut close_caller=super::close_source::caller(1,3);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let input = PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0);
        let mut job = PreparedRenderJob::new(input, 1);
        InteractiveJob::begin_close(&mut job);
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let copy = copy.as_u64().unwrap() as usize;
            let grant = RetainedCloneGrant {
                maximum_items: 1,
                maximum_copy_bytes: copy,
                maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).unwrap(),
                maximum_release_bytes: job.next_close_release_byte_demand().unwrap(),
                maximum_depth: job.next_close_depth_demand().unwrap(),
            };
            
            let (step, births, frees) = measured(|| close_caller.turn(grant, |cx| InteractiveJob::close_step(&mut job, cx)).expect("original fixture close turn"));
            let progress = match step {
                Close::Pending { progress } | Close::Complete { progress } => progress,
                Close::Blocked => panic!("requested prepared close was blocked"),
            };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete, "prepared original owner must physically finish within declared work");
        assert!(InteractiveJob::terminal_is_empty(&job));
        let (_, births, frees) = measured(|| drop(job));
        assert_eq!((births, frees), (0, 0));
    }

    close_caller.retire(super::close_source::STOCK);
}

#[test]
fn prepared_original_atlas_owner_preserves_denials_and_reports_every_physical_release() {
    let _guard = prepared_process_guard();
    let mut close_caller=super::close_source::caller(1,3);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    let atlas = &law["atlas"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let source = vec![91; atlas["byteLength"].as_u64().unwrap() as usize];
        let mut pages = atlas_fixture_owner(atlas["width"].as_u64().unwrap() as u32, atlas["height"].as_u64().unwrap() as u32, atlas["channels"].as_u64().unwrap() as u8, source.len()).unwrap();
        let mut row = 0;
        while row < pages.height() { atlas_fixture_page(&mut pages, &source, row).unwrap(); row = pages.next_row(); }
        let original = pages.page(0).unwrap().0.as_ptr();
        let payload_bytes = pages.len() * atlas["pageBytes"].as_u64().unwrap() as usize;
        let atlas_release = pages.original_owner_progress().retained_capacity_bytes + size_of::<PreparedFixedPage<PreparedRenderUpload>>();
        let (mut input, input_born, input_freed) = measured(|| PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0));
        let expected_release = atlas_release + input_born - input_freed;
        input.uploads.try_push(PreparedRenderUpload::GlyphAtlasPages { pixels: pages }).unwrap();
        let mut job = PreparedRenderJob::new(input, 1);
        assert!(job.commands.take().unwrap().terminal_is_empty());
        InteractiveJob::begin_close(&mut job);
        let copy = copy.as_u64().unwrap() as usize;
        let release = job.next_close_release_byte_demand().unwrap();
        assert_eq!(release, atlas["pageBytes"].as_u64().unwrap() as usize);
        for (items, released, depth) in [(0, release, 1), (1, release - 1, 1), (1, release, 0)] {
            let grant = RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: released, maximum_depth: depth };
            
            let (outcome, births, frees) = measured(|| close_caller.turn(grant, |cx| InteractiveJob::close_step(&mut job, cx)));
            assert!(matches!(outcome, Ok(Close::Pending { progress }) if progress == Default::default()) || matches!(&outcome, Err(failure) if failure.primary_value().is_some_and(|error| error.kind == semio_framework_value::ValueRefusalKind::DepthLimit)));
            assert_eq!((births, frees), (0, 0));
            let PreparedRenderUpload::GlyphAtlasPages { pixels } = job.input.as_ref().unwrap().uploads.get(0).unwrap() else { panic!("original atlas owner changed") };
            assert_eq!(pixels.page(0).unwrap().0.as_ptr(), original);
            assert_eq!(pixels.page(0).unwrap().0, &source[..atlas["pageBytes"].as_u64().unwrap() as usize]);
        }
        let mut total_release = 0;
        let mut total_birth = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = atlas_fixture_authority().retirement;
            
            let (step, births, frees) = measured(|| close_caller.turn(grant, |cx| InteractiveJob::close_step(&mut job, cx)).expect("original fixture close turn"));
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("funded atlas close failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            assert!(progress.copied_bytes <= grant.maximum_copy_bytes);
            total_release += frees;
            total_birth += births;
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete && InteractiveJob::terminal_is_empty(&job));
        assert_eq!(total_release, expected_release + total_birth);
        let (_, births, frees) = measured(|| drop(job));
        assert_eq!((births, frees), (0, 0));
    }

    close_caller.retire(super::close_source::STOCK);
}

#[test]
fn prepared_original_abandoned_owners_report_full_physical_grants() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    let recipe = &law["atlas"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let copy = copy.as_u64().unwrap() as usize;
        let (input, input_born, input_freed) = measured(|| PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0));
        let slot = usize::from(input.abandonment_slot);
        let expected = input_born - input_freed + size_of::<PreparedRenderInput>();
        drop(input);
        let original = PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
        assert!(!original.is_null());
        for grant in [RetainedCloneGrant::default()] {
            let (step, births, frees) = measured(|| semio_ui_prepared::retirement::prepared_close_authored(grant, PreparedRenderInput::close_abandoned_step).unwrap());
            assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
            assert_eq!((births, frees), (0, 0));
            assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
        }
        let mut released = 0;
        let mut born = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = PreparedRenderInput::next_abandoned_close_demands(copy).unwrap();
            if grant.maximum_release_bytes != 0 {
                let original = PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
                let denied = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
                let (step, births, frees) = measured(|| semio_ui_prepared::retirement::prepared_close_authored(denied, PreparedRenderInput::close_abandoned_step).unwrap());
                assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
                assert_eq!((births, frees), (0, 0));
                assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
            }
            let (step, births, frees) = measured(|| semio_ui_prepared::retirement::prepared_close_authored(grant, PreparedRenderInput::close_abandoned_step).unwrap());
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original abandoned input failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            released += frees;
            born += births;
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete);
        assert_eq!(released, expected + born);
        assert!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire).is_null());
        assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].load(Ordering::Acquire), 0);


    }
}

#[test]
fn prepared_close_gate_preserves_the_original_refusal_receipt() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["refusalReceipts"].as_array().unwrap() {
        let progress = RetainedCloneProgress { copied_items: row["copiedItems"].as_u64().unwrap() as usize, copied_bytes: row["copiedBytes"].as_u64().unwrap() as usize, retained_capacity_bytes: row["retainedCapacityBytes"].as_u64().unwrap() as usize, released_bytes: row["releasedBytes"].as_u64().unwrap() as usize };
        let original = ValueError::literal(ValueRefusalKind::InvariantViolated, "original child refusal").with_retained_progress(progress);
        let refusal = prepared_close_gate::<ValueError>(RetainedCloneGrant::default(), false, Err(original)).unwrap().expect_err("the gate keeps the original refusal");
        assert_eq!(refusal.kind, ValueRefusalKind::InvariantViolated);
        let received = refusal.retained_progress();
        let oracle = serde_json::json!({"copiedItems": received.copied_items, "copiedBytes": received.copied_bytes, "retainedCapacityBytes": received.retained_capacity_bytes, "releasedBytes": received.released_bytes});
        assert_eq!(&oracle, row);
    }
}

/// 🪢️ Admits the prepared law's return origin through its own caller's refreshed turns.
fn admit_prepared_origin(caller: &mut semio_framework_job::OriginalJobFixtureCaller, origin: &mut semio_framework_job::OriginalJobReturnOrigin, grant: RetainedCloneGrant, turns: u64) {
    for _ in 0..turns { if caller.refreshed_turn(grant, |cx| origin.admit(cx)).expect("original prepared return origin turn") { return } }
    panic!("the original prepared return origin admits inside its finite turns")
}

/// ▶️ Drives one prepared body through the admitted return origin and receives its returned outcome.
fn drive_prepared(caller: &mut semio_framework_job::OriginalJobFixtureCaller, origin: &mut semio_framework_job::OriginalJobReturnOrigin, job: &mut PreparedRenderJob, grant: RetainedCloneGrant) -> bool {
    caller.refreshed_turn(grant, |cx| {
        let receiver = origin.receiver()?;
        let mut verdict = None;
        match semio_framework_job::drive_step(job, cx, receiver, "prepared_outcome_law", semio_framework_job::InteractiveStage::InteractiveStep, &mut verdict)? {
            semio_framework_job::OriginalJobReturn::Outcome(reply) => { let mut input = Some(reply.into_descriptor()); origin.receive_outcome(cx, &mut input) }
            semio_framework_job::OriginalJobReturn::Continuation(_) => panic!("prepared owner returns semantic outcomes, never continuations"),
            _ => Ok(false),
        }
    }).expect("original prepared return turn")
}

#[test]
fn prepared_outcome_loans_preserve_original_custody_and_paid_descriptor_acknowledgement() {
    use semio_framework_job::{JobOutcomeView, OriginalJobReturnOrigin};
    let _guard = prepared_process_guard();
    let mut close_caller=super::close_source::caller(1,3);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["preparedOutcome"];
    let original = ui_contract::UI_WORKER_RETIREMENT_POLICY;
    let turns = fixture["prepared"]["maximumTurns"].as_u64().unwrap();
    let mut complete_absence = None;
    let mut fault_identity = false;
    for fault in [false, true] {
        let input = PreparedRenderInput::new(fixture["prepared"]["sceneRevision"].as_u64().unwrap(), fixture["prepared"]["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0);
        let mut job = PreparedRenderJob::new(input, 1);
        let generation = fixture["prepared"]["previewGeneration"].as_u64().unwrap() + u64::from(fault);
        let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(law["operation"].as_u64().unwrap()), semio_framework_job::Generation(generation), super::close_source::STOCK, now_ms, u64::MAX, 1 << 20);
        caller.birth_returns(super::close_source::STOCK);
        let mut origin = OriginalJobReturnOrigin::empty();
        admit_prepared_origin(&mut caller, &mut origin, super::close_source::STOCK, turns);
        let refresh = caller.turn(super::close_source::STOCK, |cx| semio_framework_job::original_return_refresh_quote(cx).map_err(Into::into)).expect("prepared return refresh quote turn");
        let bound = RetainedCloneGrant { maximum_items: original.maximum_items + refresh.copied_items, maximum_copy_bytes: original.maximum_copy_bytes + refresh.copied_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes + refresh.retained_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes + refresh.released_bytes, maximum_depth: original.maximum_depth };
        let denied = RetainedCloneGrant { maximum_items: law["deniedItems"].as_u64().unwrap() as usize, ..original };
        assert!(!drive_prepared(&mut caller, &mut origin, &mut job, denied));
        assert_eq!(serde_json::to_value(caller.business_receipt()).unwrap(), law["expected"]["deniedReceipt"]);
        assert!(job.fault().is_none());
        let mut reached = false;
        for _ in 0..turns {
            admit_prepared_origin(&mut caller, &mut origin, super::close_source::STOCK, turns);
            let received = drive_prepared(&mut caller, &mut origin, &mut job, original);
            assert!(caller.business_receipt().fits(bound));
            if !received { continue }
            let descriptor = origin.outcome().expect("received prepared outcome is held by its receiver");
            let terminal = descriptor.is_terminal();
            match job.borrow_outcome(descriptor).unwrap() {
                JobOutcomeView::Complete { state, output, .. } => complete_absence = Some((state.is_none(), output.is_none())),
                JobOutcomeView::Fault { detail, .. } => fault_identity = std::ptr::eq(detail, job.fault_payload.published().unwrap()),
                JobOutcomeView::Yield { .. } => {},
                outcome => panic!("prepared owner returned unexpected semantic outcome: {outcome:?}"),
            }
            let mut heap = (0, 0);
            let acknowledged = caller.refreshed_turn(original, |cx| { let (result, births, frees) = measured(|| origin.acknowledge_outcome(cx)); heap = (births, frees); result }).expect("original prepared acknowledgement turn");
            assert!(!acknowledged);
            assert_eq!(caller.business_receipt().copied_items, law["expected"]["acknowledgementItems"].as_u64().unwrap() as usize);
            assert!(caller.business_receipt().fits(original));
            assert_eq!(heap, (0, 0));
            assert!(caller.refreshed_turn(original, |cx| origin.acknowledge_outcome(cx)).expect("original prepared descriptor removal turn"));
            if terminal { reached = true; break }
        }
        for _ in 0..turns { if origin.terminal_is_empty() { break } caller.refreshed_turn(super::close_source::STOCK, |cx| origin.close_step(cx).map(|_| ())).expect("original prepared return origin close turn"); }
        assert!(origin.terminal_is_empty());
        drop(origin);
        caller.retire(super::close_source::STOCK);
        assert!(reached);
        InteractiveJob::begin_close(&mut job);
        let mut closed = false;
        for _ in 0..fixture["prepared"]["maximumTurns"].as_u64().unwrap() {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: original.maximum_copy_bytes, maximum_capacity_bytes: job.next_close_capacity_byte_demand(original.maximum_copy_bytes).unwrap(), maximum_release_bytes: job.next_close_release_byte_demand().unwrap(), maximum_depth: job.next_close_depth_demand().unwrap() };
            
            let (step, births, frees) = measured(|| close_caller.turn(grant, |cx| InteractiveJob::close_step(&mut job, cx)).expect("original fixture close turn"));
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("prepared funded semantic close failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            if matches!(step, Close::Complete { .. }) { closed = true; break }
        }
        assert!(closed && InteractiveJob::terminal_is_empty(&job));
        let (_, births, frees) = measured(|| drop(job));
        assert_eq!((births, frees), (0, 0));
    }
    let (state, output) = complete_absence.unwrap();
    let actual = serde_json::json!({ "deniedReceipt": fixture["preparedOutcome"]["expected"]["deniedReceipt"], "completeStateAbsent": state, "completeOutputAbsent": output, "faultLoanIdentity": fault_identity, "acknowledgementItems": 1 });
    assert_eq!(actual, law["expected"]);

    close_caller.retire(super::close_source::STOCK);
}
