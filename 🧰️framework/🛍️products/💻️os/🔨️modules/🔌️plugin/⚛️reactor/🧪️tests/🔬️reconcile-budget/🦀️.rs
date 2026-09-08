use super::*;

#[test]
fn patch_frame_limit_does_not_limit_internal_reconciliation_steps() {
    assert_eq!(reconcile_step_opportunities(512), 512);
    assert_eq!(reconcile_step_opportunities(15_640), RECONCILE_STEP_OPPORTUNITY_LIMIT as usize);
    assert_eq!(reconcile_step_opportunities(0), 1);
}

#[test]
fn reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps() {
    let instance = 991u32;
    let key = instance_lifetime::NativeCloseKey::fixture(instance, 1);
    reserve_reactor_close(key).expect("fixed reactor close admission");
    activate_reactor_close(key).expect("all reservations admitted before activation");
    let mut steps = 0usize;
    loop {
        REACTOR_CLOSE_CURSOR.with(|cursor| cursor.set(ReactorCloseRegistry::index(instance)));
        assert!(step_reactor_close().expect("one bounded reactor close opportunity"));
        steps += 1;
        if reactor_close_complete(key).expect("exact retained receipt") {
            break;
        }
        assert!(steps < 8_192, "fixed close cursor must terminate within its structural capacities");
    }
    assert!(steps > REACTOR_TASK_SLOTS + REACTOR_TIMER_SLOTS, "request, resume, task, timer, and metadata owners must retire across distinct opportunities");
    assert!(reserve_reactor_close(instance_lifetime::NativeCloseKey::fixture(instance, 2)).is_err(), "terminal receipt holds its exact slot until ACK");
    release_reactor_close(key).expect("final exact receipt release");
    assert!(reactor_close_complete(key).is_err(), "absence is not a terminal receipt");
}
