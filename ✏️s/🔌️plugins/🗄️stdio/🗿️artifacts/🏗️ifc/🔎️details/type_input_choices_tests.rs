//! 🧾️ A refused construction receipt retains newly born choice backing for original controlled retirement.
use super::*;
#[test]
fn original_declared_choice_backing_survives_posting_refusal_and_denied_retirement_axes() {
    use semio_framework_job::{OriginalConstructionDemand, admit_original_construction};
    use semio_framework_plugin::app_testing::OriginalWorkFixture;
    use semio_framework_value::retained_clone::RetainedCloneClose;
    let mut caller = OriginalWorkFixture::new();
    let stock = OriginalWorkFixture::stock();
    let mut owner = None;
    let demand = OriginalConstructionDemand { items: 1, copy_bytes: 4096, capacity_bytes: size_of::<StagedScalarChoice>(), release_bytes: 0, depth: 1 };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        caller.work_turn(stock, 1, |cx| {
            admit_original_construction(cx, &mut owner, demand, |_| {
                let mut rows = Vec::new();
                rows.try_reserve_exact(2).unwrap();
                rows.push(StagedScalarChoice { key: "T", label: "True" });
                rows.push(StagedScalarChoice { key: "F", label: "False" });
                let capacity = rows.capacity() * size_of::<StagedScalarChoice>();
                Ok((Choices::new(rows), RetainedCloneProgress { copied_items: 1, copied_bytes: 4096, retained_capacity_bytes: capacity, ..Default::default() }))
            })
        })
    });
    assert!(result.is_err());
    assert_eq!(owner.as_ref().unwrap().rows().len(), 2);
    let quoted = owner.as_ref().unwrap().next_demand(4096).unwrap();
    assert!(quoted.copy_bytes >= 2 * size_of::<Choices>() + size_of::<StagedScalarChoice>() + size_of::<usize>());
    let mut denied = stock;
    denied.maximum_copy_bytes = quoted.copy_bytes - 1;
    let before = (owner.as_ref().unwrap().rows().as_ptr(), owner.as_ref().unwrap().rows().len());
    assert!(matches!(owner.as_mut().unwrap().close_step(denied), RetirementStep::BudgetExhausted));
    assert_eq!(before, (owner.as_ref().unwrap().rows().as_ptr(), owner.as_ref().unwrap().rows().len()));

    assert!(heap.requested_bytes >= 2 * size_of::<StagedScalarChoice>());
    assert_eq!(heap.released_bytes, 0);
    caller.cancel().cancel_now();
    let mut close = RetainedCloneClose::default();
    let mut released = 0;
    for turn in 0..100000 {
        if owner.is_none() && close.is_empty() {
            break;
        }
        let demand = close.next_owner_demands::<Choices>(owner.is_some(), &None, 4096).unwrap();
        for axis in 0..5 {
            let mut denied = stock;
            match axis {
                0 => denied.maximum_items = 0,
                1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                4 => denied.maximum_depth = 0,
                _ => continue,
            };
            caller
                .turn(denied, |cx| {
                    cx.with_original_retirement(|cx| {
                        let before = cx.retained_progress();
                        semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                            if close.is_empty() {
                                close.begin_granted(&mut owner, grant)?.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "missing original choice retirement"))
                            } else {
                                close.step_granted(grant)
                            }
                        })?;
                        assert_eq!(before, cx.retained_progress());
                        Ok(())
                    })
                })
                .unwrap();
        }
        caller
            .turn(stock, |cx| {
                cx.with_original_retirement(|cx| {
                    let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        if close.is_empty() {
                            close.begin_granted(&mut owner, grant)?.ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "missing original choice retirement"))
                        } else {
                            close.step_granted(grant)
                        }
                    })?;
                    released += step.progress().released_bytes;
                    Ok(())
                })
            })
            .unwrap();
        assert!(turn < 99999);
    }
    assert!(owner.is_none() && close.is_empty());
    assert!(released >= 2 * size_of::<StagedScalarChoice>());
    caller.finish();
}
                                                                                                                                                  