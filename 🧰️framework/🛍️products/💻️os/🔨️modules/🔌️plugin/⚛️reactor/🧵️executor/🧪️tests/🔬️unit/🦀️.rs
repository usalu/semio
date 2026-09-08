use super::*;
use std::cell::Cell;
use std::task::Poll as StdPoll;

struct YieldOnce {
    yielded: bool,
}

impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
        if self.yielded {
            StdPoll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            StdPoll::Pending
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn spawn_runs_a_ready_task_to_completion() {
    let executor = ColdFutureExecutor::new();
    let ran = Rc::new(Cell::new(false));
    let ran_inner = ran.clone();
    executor
        .spawn(async move {
            ran_inner.set(true);
        })
        .expect("fixed executor admission");
    let pending = executor.run_until_idle(8);
    assert!(ran.get(), "task body must have run");
    assert!(!pending, "no task should remain pending");
}

#[semio_framework_async_macros::async_test]
async fn a_self_waking_task_is_polled_again_within_the_same_pass() {
    let executor = ColdFutureExecutor::new();
    executor.spawn(YieldOnce { yielded: false }).expect("fixed executor admission");
    let pending = executor.run_until_idle(8);
    assert!(!pending, "YieldOnce must complete within the iteration budget");
}

#[semio_framework_async_macros::async_test]
async fn a_task_that_never_wakes_stays_pending_until_woken() {
    let executor = ColdFutureExecutor::new();
    let waker_cell: Rc<RefCell<Option<Waker>>> = Rc::new(RefCell::new(None));
    let waker_cell_inner = waker_cell.clone();
    struct ParkForever {
        cell: Rc<RefCell<Option<Waker>>>,
        done: Rc<Cell<bool>>,
    }
    impl Future for ParkForever {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
            if self.done.get() {
                StdPoll::Ready(())
            } else {
                *self.cell.borrow_mut() = Some(cx.waker().clone());
                StdPoll::Pending
            }
        }
    }
    let done = Rc::new(Cell::new(false));
    let done_inner = done.clone();
    executor.spawn(ParkForever { cell: waker_cell_inner, done: done_inner }).expect("fixed executor admission");
    let pending = executor.run_until_idle(8);
    assert!(pending, "task must stay parked until its waker fires");
    assert!(!executor.has_ready(), "a parked task must not remain in the ready queue");

    done.set(true);
    waker_cell.borrow().as_ref().expect("poll must have captured a waker").wake_by_ref();
    let pending = executor.run_until_idle(8);
    assert!(!pending, "waking must let the task observe `done` and complete");
}

#[semio_framework_async_macros::async_test]
async fn cancel_before_the_first_run_until_idle_drops_the_future_without_ever_polling_it() {
    struct DropFlag(Rc<Cell<bool>>);
    impl Drop for DropFlag {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let executor = ColdFutureExecutor::new();
    let polled = Rc::new(Cell::new(false));
    let dropped = Rc::new(Cell::new(false));
    let polled_inner = polled.clone();
    let flag = DropFlag(dropped.clone());
    let id = executor
        .spawn(async move {
            let _flag = flag;
            polled_inner.set(true);
        })
        .expect("fixed executor admission");
    let detached = executor.detach(id).expect("exact detached future");
    let pending = executor.run_until_idle(8);
    assert!(!polled.get(), "a cancelled task's body must never run");
    assert!(!dropped.get(), "detaching must not synchronously drop the future");
    drop(detached);
    assert!(dropped.get(), "the explicit disposal owner controls the future's final drop");
    assert!(!pending, "nothing should remain pending after cancelling the only task");
}

#[semio_framework_async_macros::async_test]
async fn cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse() {
    let executor = ColdFutureExecutor::new();
    let waker_cell: Rc<RefCell<Option<Waker>>> = Rc::new(RefCell::new(None));
    let waker_cell_inner = waker_cell.clone();
    struct ParkForever {
        cell: Rc<RefCell<Option<Waker>>>,
    }
    impl Future for ParkForever {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> StdPoll<()> {
            *self.cell.borrow_mut() = Some(cx.waker().clone());
            StdPoll::Pending
        }
    }
    let id = executor.spawn(ParkForever { cell: waker_cell_inner }).expect("fixed executor admission");
    let pending = executor.run_until_idle(8);
    assert!(pending, "task must be parked");
    drop(executor.detach(id).expect("exact detached future"));
    assert!(!executor.has_pending(), "cancelling the only parked task must clear has_pending");
    // 🔁️ The freed slot is reused by the next spawn — cancel must not leak the index forever.
    let reused = executor.spawn(async move {}).expect("reused fixed executor slot");
    assert_ne!(reused, id, "slot reuse must advance generation authority");
    assert_eq!(reused as u32, id as u32, "a detached slot must be reusable by a later spawn");
}

#[semio_framework_async_macros::async_test]
async fn spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs() {
    let executor = ColdFutureExecutor::new();
    let seen_id: Rc<Cell<Option<TaskId>>> = Rc::new(Cell::new(None));
    let seen_id_inner = seen_id.clone();
    let id = executor
        .spawn_with_id(move |id| {
            Box::pin(async move {
                seen_id_inner.set(Some(id));
            })
        })
        .expect("fixed executor admission");
    let pending = executor.run_until_idle(8);
    assert!(!pending);
    assert_eq!(seen_id.get(), Some(id), "the future must observe the SAME id spawn_with_id returned");
}

#[semio_framework_async_macros::async_test]
async fn cancel_is_idempotent_for_an_unknown_or_already_finished_id() {
    let executor = ColdFutureExecutor::new();
    assert!(executor.detach(999).is_none());
    let id = executor.spawn(async move {}).expect("fixed executor admission");
    let _ = executor.run_until_idle(8); // finishes and frees the slot
    assert!(executor.detach(id).is_none());
    assert!(executor.detach(id).is_none());
}

#[semio_framework_async_macros::async_test]
async fn detach_and_reuse_ten_times_capacity_never_accumulates_stale_ready_authority() {
    let executor = ColdFutureExecutor::new();
    for _ in 0..LOCAL_EXECUTOR_TASK_SLOTS * 10 {
        let id = executor.spawn(async move {}).expect("one fixed task slot remains reusable");
        drop(executor.detach(id).expect("ready task detaches by exact generation"));
        let inner = executor.inner.borrow();
        assert_eq!(inner.ready_len, 0);
        assert_eq!(inner.live, 0);
        assert_eq!(inner.free.len(), LOCAL_EXECUTOR_TASK_SLOTS);
    }
}

#[semio_framework_async_macros::async_test]
async fn self_detach_during_poll_cannot_steal_or_drop_the_in_flight_future() {
    struct SelfDetach {
        executor: ColdFutureExecutor,
        id: TaskId,
        attempted: Rc<Cell<bool>>,
    }
    impl Future for SelfDetach {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> StdPoll<()> {
            self.attempted.set(true);
            assert!(self.executor.detach(self.id).is_none(), "an in-flight future is detached from its slot while polled");
            StdPoll::Ready(())
        }
    }

    let executor = ColdFutureExecutor::new();
    let attempted = Rc::new(Cell::new(false));
    let reservation = executor.reserve().expect("fixed reservation");
    let id = reservation.id();
    reservation.install(Box::pin(SelfDetach { executor: executor.clone(), id, attempted: attempted.clone() }));
    assert!(!executor.run_until_idle(1));
    assert!(attempted.get());
    assert!(!executor.has_pending());
}

#[semio_framework_async_macros::async_test]
async fn reactor_task_close_releases_one_nested_owner_per_step_and_only_then_drops_terminal_shell() {
    struct DropItem(Rc<Cell<usize>>);
    impl Drop for DropItem {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    struct BoundedTask {
        items: Vec<DropItem>,
    }
    impl ReactorTask for BoundedTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Complete
        }

        fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep {
            if budget.maximum_units == 0 || std::time::Instant::now() >= budget.deadline {
                return ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 };
            }
            if self.items.pop().is_some() {
                ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
            } else {
                ReactorTaskStep::Complete
            }
        }

        fn terminal_is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    let executor = ReactorExecutor::new();
    assert!(executor.pre_admit());
    let dropped = Rc::new(Cell::new(0));
    let task = BoundedTask { items: (0..3).map(|_| DropItem(dropped.clone())).collect() };
    assert!(executor.admit(17, 91, 4, 4, Box::new(task)).is_ok(), "fixed reactor task admission");
    assert!(executor.run_until_deadline(1, 0, std::time::Instant::now() + std::time::Duration::from_millis(8)));
    let mut cursor = 0;
    let zero = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 0, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
    assert_eq!(executor.close_instance_step(17, &mut cursor, zero), ReactorTaskStep::Pending { processed_units: 0, processed_bytes: 0 });
    assert_eq!(dropped.get(), 0, "zero fuel cannot release nested owners");
    for expected in 1..=3 {
        let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
        assert_eq!(executor.close_instance_step(17, &mut cursor, budget), ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 });
        assert_eq!(dropped.get(), expected, "one close step releases exactly one nested owner");
    }
    let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
    assert!(matches!(executor.close_instance_step(17, &mut cursor, budget), ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 } | ReactorTaskStep::Complete));
    assert!(!executor.has_pending());
    assert_eq!(dropped.get(), 3, "terminal task drop is empty and constant-time");
}

#[semio_framework_async_macros::async_test]
async fn rejected_reactor_task_is_bounded_disposed_without_drop() {
    struct RejectedTask {
        remaining: usize,
    }
    impl ReactorTask for RejectedTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Blocked { reason: "not admitted" }
        }

        fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            if self.remaining == 0 {
                ReactorTaskStep::Complete
            } else {
                self.remaining -= 1;
                ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
            }
        }

        fn terminal_is_empty(&self) -> bool {
            self.remaining == 0
        }
    }

    let executor = ReactorExecutor::new();
    executor.inner.borrow_mut().allocation_admitted = false;
    let mut rejected = match executor.admit(1, 2, 3, 3, Box::new(RejectedTask { remaining: 2 })) {
        Ok(_) => panic!("forced admission failure was accepted"),
        Err(rejected) => rejected,
    };
    let budget = ReactorTaskBudget { operation: 2, generation: 3, cancellation_generation: 3, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
    assert!(matches!(rejected.close_step(budget), ReactorTaskStep::Pending { .. }));
    assert!(matches!(rejected.close_step(budget), ReactorTaskStep::Pending { .. }));
    assert_eq!(rejected.close_step(budget), ReactorTaskStep::Complete);
}

#[semio_framework_async_macros::async_test]
async fn blocked_reactor_task_does_not_starve_ready_peer() {
    struct BlockedTask;
    impl ReactorTask for BlockedTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Blocked { reason: "external wait" }
        }

        fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            true
        }
    }
    struct ReadyTask(Rc<Cell<bool>>);
    impl ReactorTask for ReadyTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            self.0.set(true);
            ReactorTaskStep::Complete
        }

        fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            true
        }
    }

    let executor = ReactorExecutor::new();
    assert!(executor.pre_admit());
    let blocked_id = match executor.admit(1, 1, 1, 1, Box::new(BlockedTask)) {
        Ok(id) => id,
        Err(_) => panic!("blocked fixture admission failed"),
    };
    let ran = Rc::new(Cell::new(false));
    assert!(executor.admit(1, 2, 1, 1, Box::new(ReadyTask(ran.clone()))).is_ok());
    executor.run_until_deadline(4, 0, std::time::Instant::now() + std::time::Duration::from_millis(8));
    assert!(ran.get(), "a blocked slot cannot end the bounded scan before an unrelated ready slot");
    assert!(executor.cancel(blocked_id, 2));
    let mut cursor = 0;
    let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
    executor.close_instance_step(1, &mut cursor, budget);
    executor.close_instance_step(1, &mut cursor, budget);
    assert!(!executor.has_pending());
}

#[semio_framework_async_macros::async_test]
async fn stale_generation_cannot_commit() {
    struct GenerationTask(Rc<Cell<bool>>);
    impl ReactorTask for GenerationTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            panic!("cancelled generation reached normal step")
        }

        fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep {
            self.0.set(budget.generation == 7 && budget.cancellation_generation == 8);
            ReactorTaskStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            true
        }
    }

    let executor = ReactorExecutor::new();
    assert!(executor.pre_admit());
    let observed = Rc::new(Cell::new(false));
    let id = match executor.admit(1, 9, 7, 7, Box::new(GenerationTask(observed.clone()))) {
        Ok(id) => id,
        Err(_) => panic!("fixed generation task admission failed"),
    };
    assert!(executor.cancel(id, 8));
    executor.run_until_deadline(1, 0, std::time::Instant::now() + std::time::Duration::from_millis(8));
    assert!(observed.get());
    assert!(!executor.has_pending());
}

#[semio_framework_async_macros::async_test]
async fn reactor_executor_shutdown_drains_every_slot_before_terminal_drop() {
    struct EmptyTask;
    impl ReactorTask for EmptyTask {
        fn step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Pending { processed_units: 1, processed_bytes: 0 }
        }

        fn close_step(&mut self, _budget: ReactorTaskBudget) -> ReactorTaskStep {
            ReactorTaskStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            true
        }
    }

    let executor = ReactorExecutor::new();
    assert!(executor.pre_admit());
    for operation in 0..LOCAL_EXECUTOR_TASK_SLOTS as u64 {
        assert!(executor.admit(1, operation, 1, 1, Box::new(EmptyTask)).is_ok());
    }
    executor.begin_shutdown();
    let budget = ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 0, deadline: std::time::Instant::now() + std::time::Duration::from_secs(5) };
    for _ in 0..LOCAL_EXECUTOR_TASK_SLOTS * 3 + 1 {
        if executor.shutdown_step(budget) == ReactorTaskStep::Complete {
            break;
        }
    }
    let inner = executor.inner.borrow();
    assert_eq!(inner.live, 0);
    assert!(inner.free.is_empty());
    assert!(inner.slots.is_empty());
}
