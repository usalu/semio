//! 💳️ Compares the combined grant observation with the separate quote, post and debit against live Source stock.

use super::{quantum, stock};
use semio_framework_job_context::{root_cancel_token, ClockStride, Generation, OperationId, OriginalCallerChildSource, OriginalCallerReturnPlace, RetainedCloneGrant, RetainedCloneProgress, StepBudget, StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::ValueError;
use serde_json::Value;
use std::cell::Cell;

thread_local! {
    static NOW: Cell<Option<u64>> = const { Cell::new(Some(1)) };
    static SAMPLES: Cell<usize> = const { Cell::new(0) };
}

fn clock() -> Option<u64> {
    SAMPLES.set(SAMPLES.get() + 1);
    NOW.get()
}

fn old_charge(cx: &mut StepContext<'_>, progress: RetainedCloneProgress, retirement: bool) -> Result<bool, ValueError> {
    let grant = cx.retained_grant();
    let yielded = if retirement { cx.retirement_should_yield() } else { cx.should_yield() };
    if yielded || grant.maximum_depth == 0 || !progress.fits(grant) {
        return Ok(false);
    }
    cx.consume_retained(progress)?;
    cx.consume_fuel(1);
    Ok(true)
}

fn new_charge(cx: &mut StepContext<'_>, progress: RetainedCloneProgress, retirement: bool) -> Result<bool, ValueError> {
    let RetainedCloneProgress { copied_bytes, retained_capacity_bytes, released_bytes, .. } = progress;
    if retirement {
        cx.charge_retirement_retained_item(copied_bytes, retained_capacity_bytes, released_bytes)
    } else {
        cx.charge_retained_item(copied_bytes, retained_capacity_bytes, released_bytes)
    }
}

fn grant(row: &Value) -> RetainedCloneGrant {
    let n = |i: usize| row[i].as_u64().unwrap() as usize;
    RetainedCloneGrant { maximum_items:n(0), maximum_copy_bytes:n(1), maximum_capacity_bytes:n(2), maximum_release_bytes:n(3), maximum_depth:n(4) }
}

fn progress(row: &Value) -> RetainedCloneProgress {
    let n = |i: usize| row[i].as_u64().unwrap() as usize;
    RetainedCloneProgress { copied_items:1, copied_bytes:n(0), retained_capacity_bytes:n(1), released_bytes:n(2) }
}

fn differential_witness(cases: &Value, fast: bool) -> Vec<String> {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(924), Generation(1), stock(), clock, u64::MAX, 100_000);
    let mut child = caller.turn(quantum(), |cx| {
        Ok(OriginalCallerChildSource::new(cx.original_source_parent()?, cx, OperationId(925), Generation(1))?)
    }).unwrap();
    for _ in 0..16 {
        if child.parent().is_ok() { break; }
        caller.turn(quantum(), |cx| child.admit_step(cx.original_source_parent()?, cx)).unwrap();
    }
    caller.birth_returns(quantum());
    caller.retain_dispatch().unwrap();
    let destination = [0u8; 4];
    let address = destination.as_ptr() as usize;
    let mut place = OriginalCallerReturnPlace::empty();
    for _ in 0..16 {
        if caller.refreshed_turn(quantum(), |cx| place.admit_sized(cx, address, destination.len())).unwrap() { break; }
    }
    assert!(place.is_admitted());
    let charge = if fast { new_charge } else { old_charge };
    let mut observations = Vec::new();
    for row in cases.as_array().unwrap() {
        let receipt = progress(&row["quote"]);
        let loan = row["loan"].as_bool().unwrap();
        let retirement = row["retirement"].as_bool().unwrap();
        let delivered = caller.refreshed_turn(quantum(), |parent_cx| {
            let returned = child.with_context(parent_cx.original_source_parent()?, parent_cx, quantum(), |cx| {
                assert!(cx.original_return_budget_is_borrowed());
                let budget = StepBudget::new(cx.original_turn_fuel_remaining(), cx.deadline_us(), grant(&row["grant"]));
                cx.narrow_original_work_budget(budget)?;
                let mut perform = |cx: &mut StepContext<'_>| {
                    cx.restore_clock_stride(ClockStride::new());
                    let before = cx.retained_progress();
                    let prior_work = cx.original_work_units_spent();
                    let bank = cx.original_return_budget().unwrap().get();
                    let prior = bank.snapshot();
                    let (raw, raw_fuel) = bank.raw_remaining()?;
                    let (net, net_fuel) = bank.remaining()?;
                    assert_eq!(raw.maximum_items - net.maximum_items, 2);
                    assert_eq!(raw.maximum_copy_bytes - net.maximum_copy_bytes, prior.held_copy_bytes);
                    assert_eq!(raw_fuel - net_fuel, 2);
                    let reserved = (cx.reserved_return_progress(), cx.reserved_return_work());
                    NOW.set(row["clock"].as_u64());
                    SAMPLES.set(0);
                    let actual = charge(cx, receipt, retirement)?;
                    let sampled = SAMPLES.get();
                    NOW.set(Some(1));
                    assert_eq!(actual, row["accepted"].as_bool().unwrap(), "{}", row["name"]);
                    assert_eq!(sampled, usize::from(row["sampled"].as_bool().unwrap()), "{}", row["name"]);
                    let delta = if actual { receipt } else { RetainedCloneProgress::default() };
                    assert_eq!(cx.retained_progress(), before.checked_add(delta)?);
                    assert_eq!(cx.original_work_units_spent(), prior_work + u128::from(actual));
                    assert_eq!((cx.reserved_return_progress(), cx.reserved_return_work()), reserved);
                    let bank = cx.original_return_budget().unwrap().get();
                    let after = bank.snapshot();
                    assert_eq!(after.spent, prior.spent.checked_add(delta)?);
                    assert_eq!(after.work, prior.work + u128::from(actual));
                    assert!(after.conserved());
                    observations.push(format!("{actual} {sampled} {:?} {:?} {:?} {:?} {:?}", cx.clock_stride(), cx.retained_grant(), cx.fuel_remaining(), bank.remaining(), after));
                    Ok(())
                };
                cx.with_original_work_loan(row["fuel"].as_u64().unwrap(), 10, |cx| {
                    if loan { cx.with_original_retirement(perform) } else { perform(cx) }
                })
            })?;
            returned.into_original().map_err(|_| ValueError::invariant("borrowed charge receipt refused"))??;
            Ok(())
        });
        delivered.unwrap();
        observations.push(format!("{:?}", caller.business_receipt()));
    }
    for _ in 0..16 {
        if caller.refreshed_turn(quantum(), |cx| place.settle_return(cx)).unwrap() { break; }
    }
    assert!(place.terminal_is_empty());
    let count = 20_000;
    let slice = RetainedCloneGrant { maximum_items: count, ..quantum() };
    caller.refreshed_turn(slice, |parent_cx| {
        let returned = child.with_context(parent_cx.original_source_parent()?, parent_cx, slice, |cx| {
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            let before = cx.original_return_budget().unwrap().get().snapshot();
            assert_eq!(before.claims, 0);
            for _ in 0..count {
                assert!(charge(cx, progress, false)?);
            }
            let after = cx.original_return_budget().unwrap().get().snapshot();
            assert_eq!(after.spent.copied_items - before.spent.copied_items, count);
            assert_eq!(after.work - before.work, count as u128);
            assert!(after.conserved());
            Ok(())
        })?;
        returned.into_original().map_err(|_| ValueError::invariant("borrowed charge benchmark receipt refused"))?
    }).unwrap();
    for _ in 0..64 {
        if child.terminal_is_empty() { break; }
        caller.turn(quantum(), |cx| child.close_step(cx.original_source_parent()?, cx)).unwrap();
    }
    assert!(child.terminal_is_empty());
    let starting = RetainedCloneProgress { copied_items: 2, copied_bytes: 3, ..Default::default() };
    let (mut physical, mut work, mut reserved, mut reserved_work) = (starting, 5, starting, 7);
    let remaining;
    let unit = RetainedCloneProgress { copied_items: 1, ..Default::default() };
    {
        let mut cx = caller.lend_context();
        assert!(!cx.original_return_budget_is_borrowed());
        assert!(cx.original_return_budget().unwrap().get().is_bound());
        cx.bind_original_physical_recipient(&mut physical).unwrap();
        cx.bind_original_work_recipient(&mut work).unwrap();
        cx.bind_original_reserved_recipient(&mut reserved, &mut reserved_work).unwrap();
        assert!(charge(&mut cx, unit, false).unwrap());
        remaining = cx.original_source_fuel_remaining();
    }
    assert_eq!((physical, work, reserved, reserved_work), (starting.checked_add(unit).unwrap(), 6, starting, 7));
    caller.settle_lent_fuel(remaining);
    caller.retire(stock());
    observations
}

#[test]
fn retained_charge_matches_original_quote_and_post_with_live_source() {
    let cases = serde_json::from_str(include_str!("🧫️fixtures/retained-charge.json")).unwrap();
    assert_eq!(differential_witness(&cases, false), differential_witness(&cases, true));
}

#[test]
fn retained_charge_preserves_physical_refusal_and_unspent_fuel() {
    use semio_framework_value::ValueRefusalKind;
    let charges: [fn(&mut StepContext<'_>, RetainedCloneProgress, bool) -> Result<bool, ValueError>; 2] = [old_charge, new_charge];
    for charge in charges {
        let mut retained = RetainedCloneProgress::default();
        let mut sequence = 0;
        let budget = StepBudget::new(1, 10, quantum());
        let mut cx = StepContext::new(OperationId(926), Generation(1), budget, root_cancel_token(), clock, &mut sequence, &mut retained);
        let initial = RetainedCloneProgress { copied_bytes: budget.retained.maximum_copy_bytes + 1, ..Default::default() };
        assert!(cx.consume_retained(initial).is_err());
        let zero_copy = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        assert_eq!(charge(&mut cx, zero_copy, false), Err(ValueError::invariant("job child receipt exceeds original remaining retained grant").with_retained_progress(zero_copy)));
        assert_eq!(cx.retained_progress(), initial.checked_add(zero_copy).unwrap());
        assert_eq!(cx.fuel_remaining(), 1);
        drop(cx);

        retained = RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(927), Generation(1), budget, root_cancel_token(), clock, &mut sequence, &mut retained);
        let reserved = RetainedCloneProgress { copied_bytes: usize::MAX, ..Default::default() };
        cx.receive_original_reserved_return(reserved, 0).unwrap();
        let copy = RetainedCloneProgress { copied_bytes: 1, ..zero_copy };
        assert_eq!(charge(&mut cx, copy, false), Err(ValueError::literal(ValueRefusalKind::WorkLimit, "retained clone byte progress overflow").with_retained_progress(copy)));
        assert_eq!(cx.retained_progress(), reserved);
        assert_eq!(cx.fuel_remaining(), 1);
    }
}

#[test]
fn retained_charge_preserves_cached_then_sampled_deadline() {
    let charges: [fn(&mut StepContext<'_>, RetainedCloneProgress, bool) -> Result<bool, ValueError>; 2] = [old_charge, new_charge];
    for charge in charges {
        NOW.set(Some(9));
        let mut retained = RetainedCloneProgress::default();
        let mut sequence = 0;
        let budget = StepBudget::new(2, 10, quantum());
        let mut cx = StepContext::new(OperationId(928), Generation(1), budget, root_cancel_token(), clock, &mut sequence, &mut retained);
        assert_eq!(cx.now_us(), Some(9));
        assert_eq!(cx.now_us(), Some(9));
        assert_eq!(cx.clock_stride().stride(), 2);
        NOW.set(Some(10));
        SAMPLES.set(0);
        let unit = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        assert_eq!(charge(&mut cx, unit, false), Ok(true));
        assert_eq!(SAMPLES.get(), 0);
        assert_eq!(charge(&mut cx, unit, false), Ok(false));
        assert_eq!(SAMPLES.get(), 1);
        assert_eq!(cx.retained_progress(), unit);
        assert_eq!(cx.fuel_remaining(), 1);
    }
}

/// 🌱️ The external work recipient posts only the business view; resident birth work excluded from it never reaches the issuer's next-epoch fuel check. The fixture is left unclosed on purpose: closure and its heap witness belong to the other tests of this binary.
#[test]
fn work_recipient_posts_the_business_view_without_resident_birth_work() {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(929), Generation(1), stock(), clock, u64::MAX, 100_000);
    let (mut physical, mut work, mut reserved, mut reserved_work) = (RetainedCloneProgress::default(), 5, RetainedCloneProgress::default(), 7);
    let (business, remaining);
    {
        let mut cx = caller.lend_context();
        cx.bind_original_physical_recipient(&mut physical).unwrap();
        cx.bind_original_work_recipient(&mut work).unwrap();
        cx.bind_original_reserved_recipient(&mut reserved, &mut reserved_work).unwrap();
        cx.consume_fuel(3);
        let baseline = cx.original_birth_baseline();
        cx.consume_fuel(2);
        cx.receive_original_reserved_return(RetainedCloneProgress { copied_items: 1, copied_bytes: 8, ..Default::default() }, 4).unwrap();
        cx.exclude_original_birth(baseline);
        business = cx.original_work_units_spent();
        remaining = cx.original_source_fuel_remaining();
    }
    assert_eq!(business, 3 + 4, "the redeemed work stays in the view the reserved observer subtracts from");
    assert_eq!(work, 5 + business);
    assert_eq!(reserved_work, 7 + 4);
    assert_eq!(reserved.copied_bytes, 8);
    caller.settle_lent_fuel(remaining);
    std::mem::forget(caller);
}
