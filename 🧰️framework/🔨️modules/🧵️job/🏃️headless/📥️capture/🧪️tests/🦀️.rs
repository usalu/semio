//! 🧪️ Portable capture frontiers observe actual allocation, bounded copies and original-source return.
use super::*;
use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_value_testing_allocator::{observe_retirement_allocations, refuse_one_allocation_on_this_thread};

fn site() -> HeadlessSite {
    HeadlessSite { site: "headless.capture.test", label: "headless-capture-test", stage: InteractiveStage::InteractiveStep, fuel_per_step: 64, step_budget_us: 1000, retained_stock: RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 8 << 20, maximum_capacity_bytes: 8 << 20, maximum_release_bytes: 8 << 20, maximum_depth: 256 }, lifetime_fuel: 10_000_000, lifetime_budget_us: 1_000_000 }
}

fn caller(operation: Operation, site: &HeadlessSite) -> HeadlessCaller {
    let mut caller = HeadlessCaller::new(operation, || Some(0), site).unwrap();
    for _ in 0..32 { if caller.admit_step().unwrap() { return caller; } }
    panic!("original capture caller admission stalled");
}

fn delta(before: RetainedCloneProgress, after: RetainedCloneProgress) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: after.copied_items - before.copied_items, copied_bytes: after.copied_bytes - before.copied_bytes, retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes, released_bytes: after.released_bytes - before.released_bytes }
}

fn measured<T: Copy>(caller: &mut HeadlessCaller, site: &HeadlessSite, retiring: bool, grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> (Result<T, OriginalTurnFailure>, RetainedCloneProgress, u128) {
    let before = caller.receipt();
    let mut work = 0;
    let (result, heap) = observe_retirement_allocations(|| caller.turn(site, retiring, |cx| {
        let initial = cx.original_work_units_spent();
        let result = cx.with_retained_loan(grant, |same| same.with_original_work_loan(1, same.original_absolute_deadline_us(), run));
        work = cx.original_work_units_spent() - initial;
        result
    }));
    let progress = delta(before, caller.receipt());
    assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), heap);
    assert!(progress.fits(grant));
    assert!(work <= 1, "capture cannot refill one original work unit");
    (result, progress, work)
}

fn build(caller: &mut HeadlessCaller, site: &HeadlessSite, builder: &mut RetainedPayloadBuilder, bytes: &[u8]) {
    let mut cursor = 0;
    for _ in 0..10_000 {
        let complete = caller.turn(site, false, |cx| {
            let turn = if !builder.is_initialized() { builder.advance_initialization(cx) }
                else if cursor < bytes.len() { builder.append_original(cx, bytes, &mut cursor) }
                else if builder.published().is_none() { builder.seal(cx) }
                else { return Ok(true); };
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            Ok(builder.published().is_some())
        }).unwrap();
        if complete { return; }
    }
    panic!("original test payload did not seal");
}

fn finish_capture(capture: &mut PayloadCapture, caller: &mut HeadlessCaller, site: &HeadlessSite, payload: Option<&RetainedJobPayload>, grant: RetainedCloneGrant) -> (usize, usize, usize) {
    let mut turns = 0;
    let mut copies = 0;
    let mut born = 0;
    for _ in 0..10_000 {
        let (result, progress, _) = measured(caller, site, false, grant, |cx| capture.advance(payload, cx));
        turns += 1;
        copies += usize::from(progress.copied_bytes != 0);
        born += progress.retained_capacity_bytes;
        if result.unwrap() { return (turns, copies, born); }
    }
    panic!("bounded terminal capture stalled");
}

fn close_capture(capture: &mut PayloadCapture, caller: &mut HeadlessCaller, site: &HeadlessSite, grant: RetainedCloneGrant) -> usize {
    let mut released = 0;
    for _ in 0..16 {
        if capture.terminal_is_empty() { return released; }
        let quote = capture.retirement_demands();
        assert!(quote.release_bytes <= grant.maximum_release_bytes);
        let (result, progress, _) = measured(caller, site, true, grant, |cx| capture.close_step(cx));
        result.unwrap();
        released += progress.released_bytes;
    }
    panic!("original capture did not retire");
}

fn close_builder(builder: &mut RetainedPayloadBuilder, caller: &mut HeadlessCaller, site: &HeadlessSite) {
    for _ in 0..10_000 {
        if builder.terminal_is_empty() { return; }
        caller.turn(site, true, |cx| semio_framework_job::close_original_retained_step(cx, builder.retirement_demands()?, |grant| builder.close_step_granted(grant))).unwrap();
    }
    panic!("original test payload did not retire");
}

fn close_caller(caller: &mut HeadlessCaller) {
    for _ in 0..64 { if caller.terminal_is_empty() { return; } caller.close_step().unwrap(); }
    panic!("capture source did not retire");
}

#[test]
fn portable_capture_frontiers_preserve_bytes_identity_and_physical_receipts() {
    let law: serde_json::Value = serde_json::from_str(include_str!("frontiers.json")).unwrap();
    assert_eq!(JOB_PAYLOAD_PAGE_BYTES, law["pageBytes"].as_u64().unwrap() as usize);
    let length = law["length"].as_u64().unwrap() as usize;
    let modulus = law["modulus"].as_u64().unwrap() as usize;
    let data: Vec<u8> = (0..length).map(|index| (index % modulus) as u8).collect();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: law["copyBytes"].as_u64().unwrap() as usize, ..HEADLESS_GRANT };
    for case in law["cases"].as_array().unwrap() {
        let case = case.as_str().unwrap();
        let site = site();
        let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), Generation(1), 7);
        let mut caller = caller(operation, &site);
        let mut builder = RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput);
        let mut other = RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput);
        build(&mut caller, &site, &mut builder, &data);
        build(&mut caller, &site, &mut other, &data);
        let payload = builder.published().unwrap();
        assert_eq!(payload.page_count(), law["expected"]["pages"].as_u64().unwrap() as usize);
        let mut capture = PayloadCapture::new();
        let mut born = 0;
        match case {
            "capacityDenied" => {
                let denied = RetainedCloneGrant { maximum_capacity_bytes: length - 1, ..grant };
                let (result, progress, work) = measured(&mut caller, &site, false, denied, |cx| capture.advance(Some(payload), cx));
                assert!(!result.unwrap());
                assert_eq!(progress, RetainedCloneProgress::default());
                assert_eq!(work, 0);
                assert!(capture.terminal_is_empty());
            }
            "allocationRefusal" => {
                measured(&mut caller, &site, false, grant, |cx| capture.advance(Some(payload), cx)).0.unwrap();
                let mut refusals = 0;
                let (result, progress, work) = measured(&mut caller, &site, false, grant, |cx| {
                    let (result, count) = refuse_one_allocation_on_this_thread(|| capture.advance(Some(payload), cx));
                    refusals = count;
                    result
                });
                assert!(result.is_err());
                assert_eq!(refusals, 1);
                assert_eq!(progress, RetainedCloneProgress::default());
                assert_eq!(work, 1);
                assert!(capture.bytes.is_none());
                assert!(!capture.terminal_is_empty());
            }
            "absentPayload" => {
                let (turns, copies, capacity) = finish_capture(&mut capture, &mut caller, &site, None, grant);
                assert_eq!((turns, copies, capacity), (2, 0, 0));
                assert!(capture.bytes.as_ref().unwrap().is_empty());
            }
            "cancelledCopy" => {
                for _ in 0..3 { born += measured(&mut caller, &site, false, grant, |cx| capture.advance(Some(payload), cx)).1.retained_capacity_bytes; }
                let before = capture.bytes.as_ref().unwrap().len();
                assert!(before > 0 && before < length);
                let (result, progress, work) = measured(&mut caller, &site, false, grant, |cx| { cx.observe_original_cancellation()?; capture.advance(Some(payload), cx) });
                assert!(!result.unwrap());
                assert_eq!(capture.bytes.as_ref().unwrap().len(), before);
                assert_eq!(progress, RetainedCloneProgress::default());
                assert_eq!(work, 0);
            }
            _ => {
                let (turns, copies, capacity) = finish_capture(&mut capture, &mut caller, &site, Some(payload), grant);
                born = capacity;
                assert_eq!(turns, law["expected"]["advanceTurns"].as_u64().unwrap() as usize);
                assert_eq!(copies, law["expected"]["copyTurns"].as_u64().unwrap() as usize);
                let bytes = capture.bytes.as_ref().unwrap();
                assert_eq!(bytes, &data);
                assert_eq!(bytes.iter().map(|byte| u64::from(*byte)).sum::<u64>(), law["expected"]["checksum"].as_u64().unwrap());
                if case == "foreignPayload" {
                    let (result, progress, work) = measured(&mut caller, &site, false, grant, |cx| capture.advance(other.published(), cx));
                    assert!(result.is_err());
                    assert_eq!((progress, work), (RetainedCloneProgress::default(), 0));
                } else if case == "foreignCaller" {
                    let mut foreign = self::caller(operation, &site);
                    let (result, progress, work) = measured(&mut foreign, &site, false, grant, |cx| capture.advance(Some(payload), cx));
                    assert!(result.is_err());
                    assert_eq!((progress, work), (RetainedCloneProgress::default(), 0));
                    close_caller(&mut foreign);
                } else if case == "releaseDenied" {
                    let denied = RetainedCloneGrant { maximum_release_bytes: capacity - 1, ..grant };
                    let (result, progress, work) = measured(&mut caller, &site, true, denied, |cx| capture.close_step(cx));
                    assert!(matches!(result.unwrap(), InteractiveJobCloseStep::Pending { .. }));
                    assert_eq!((progress, work), (RetainedCloneProgress::default(), 0));
                    assert!(capture.bytes.is_some());
                } else {
                    let (result, progress, work) = measured(&mut caller, &site, false, grant, |cx| capture.advance(Some(payload), cx));
                    assert!(result.unwrap());
                    assert_eq!((progress, work), (RetainedCloneProgress::default(), 0));
                }
            }
        }
        assert_eq!(close_capture(&mut capture, &mut caller, &site, grant), born);
        let (closed, progress, work) = measured(&mut caller, &site, true, grant, |cx| capture.close_step(cx));
        assert!(matches!(closed.unwrap(), InteractiveJobCloseStep::Complete { .. }));
        assert_eq!((progress, work), (RetainedCloneProgress::default(), 0));
        close_builder(&mut other, &mut caller, &site);
        close_builder(&mut builder, &mut caller, &site);
        close_caller(&mut caller);
    }
}
