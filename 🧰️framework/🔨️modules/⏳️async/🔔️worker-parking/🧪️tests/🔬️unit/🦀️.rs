use super::*;
use std::sync::Arc;
use std::time::Instant;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("strict worker-parking fixture")
}

fn wake_name(wake: ParkWake) -> &'static str {
    match wake {
        ParkWake::Signalled => "signalled",
        ParkWake::Woken => "woken",
        ParkWake::TimerDue => "timer-due",
        ParkWake::Closed => "closed",
    }
}

fn wait_until_parked(parking: &WorkerParking) {
    let deadline = Instant::now() + std::time::Duration::from_secs(10);
    while parking.idle.load(Ordering::SeqCst) == 0 {
        assert!(Instant::now() < deadline, "worker never parked");
        std::thread::yield_now();
    }
    drop(parking.state.lock().unwrap_or_else(PoisonError::into_inner));
}

fn run_case(case: &str, timer_due: bool) -> ParkWake {
    let parking = Arc::new(WorkerParking::new());
    let due = timer_due.then_some(if case == "keeper-deadline-elapses" { Duration::from_millis(5) } else { Duration::from_secs(600) });
    match case {
        "signal-before-park" => {
            let observed = parking.observe();
            parking.signal_work();
            parking.park(observed, due)
        }
        "close-before-park" => {
            parking.close();
            parking.park(parking.observe(), due)
        }
        "keeper-deadline-elapses" => parking.park(parking.observe(), due),
        "firing-earlier-deadline-rearms-stale-keeper" => {
            let keeper = Arc::clone(&parking);
            let observed = parking.observe();
            let handle = std::thread::spawn(move || keeper.park(observed, due));
            wait_until_parked(&parking);
            parking.signal_timer_from_firing_worker();
            parking.hand_off_timers();
            handle.join().expect("parked keeper")
        }
        _ => {
            let sleeper = Arc::clone(&parking);
            let observed = parking.observe();
            let handle = std::thread::spawn(move || sleeper.park(observed, due));
            wait_until_parked(&parking);
            parking.signal_work();
            handle.join().expect("parked worker")
        }
    }
}

#[test]
fn worker_parking_protocol_matches_the_neutral_fixture() {
    for row in fixture()["protocol"].as_array().expect("protocol rows") {
        let case = row["case"].as_str().unwrap();
        let actual = run_case(case, row["timerDue"].as_bool().unwrap());
        assert_eq!(wake_name(actual), row["expected"].as_str().unwrap(), "{case}");
    }
}

#[test]
fn an_ancestor_cancel_wakes_a_descendant_waiter_once_and_drop_retains_nothing() {
    struct CountWake(AtomicUsize);
    impl std::task::Wake for CountWake {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let cancellation = &fixture()["cancellation"];
    let depth = cancellation["chainDepth"].as_u64().unwrap() as usize;
    let mut chain = vec![crate::CancelToken::root_now()];
    for _ in 1..depth {
        let child = chain.last().unwrap().child_now();
        chain.push(child);
    }
    let counter = Arc::new(CountWake(AtomicUsize::new(0)));
    let waker = std::task::Waker::from(Arc::clone(&counter));
    let mut context = std::task::Context::from_waker(&waker);
    let mut waiting = Box::pin(chain.last().unwrap().cancelled());
    assert!(std::future::Future::poll(waiting.as_mut(), &mut context).is_pending());
    assert!(std::future::Future::poll(waiting.as_mut(), &mut context).is_pending());
    chain[cancellation["cancelLevel"].as_u64().unwrap() as usize].cancel_now();
    assert_eq!(counter.0.load(Ordering::SeqCst) as u64, cancellation["expectedWakes"].as_u64().unwrap());
    assert!(std::future::Future::poll(waiting.as_mut(), &mut context).is_ready());
    drop(waiting);
    let retained: usize = chain.iter().map(|token| token.0.waiters.lock().unwrap().len()).sum();
    assert_eq!(retained as u64, cancellation["retainedWaitersAfterDrop"].as_u64().unwrap());
    let live = crate::CancelToken::root_now();
    let mut abandoned = Box::pin(live.child_now().cancelled());
    assert!(std::future::Future::poll(abandoned.as_mut(), &mut context).is_pending());
    drop(abandoned);
    assert_eq!(live.0.waiters.lock().unwrap().len(), 0, "an abandoned waiter leaves no registration behind");
}
