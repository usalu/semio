/// 🧩️ Task admission and cancellation: the deferred user factory, the spawn quota gate, key dedupe cancellation and instance close cancellation.

    /// 🧵️ Defers the original task factory until the actor polls its admitted future.
    #[semio_framework_async_macros::async_test]
    async fn task_admission_defers_the_user_factory_until_actor_execution() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../⚛️reactor/🧵️task/🧪️tests/🧫️fixtures/📥️admission/🔣️.json")).expect("canonical task admission fixture");
        let expected = |phase: &str| fixture["observations"].as_array().expect("observations").iter().find(|row| row["phase"] == phase).expect("phase")["invokesTaskFactory"].as_bool().expect("factory invocation");
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory_calls = calls.clone();
        let instance = 510;
        let meta = ActionMeta { actor: "factory-owner".into(), instance_id: instance, view_state: None };
        let task = test_task("deferred-factory", move |_ctx| {
            factory_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move { Ok(TaskResolution::Done) }
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).expect("original task admitted");
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), usize::from(expected("admission")));
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 1);
        crate::reactor_driver::run_until_idle(8).await;
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), usize::from(expected("execution")));
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 0);
    }

    impl<F, Fut> AsyncTaskSource<TestMutation, TestConfigMutation, NoDraftMutation> for TestTaskSource<F>
    where
        F: FnOnce(TaskCtx) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<TaskResolution<TestMutation, TestConfigMutation, NoDraftMutation>, Fault>> + 'static,
    {
        type Future = Fut;
        fn run(self, context: TaskCtx) -> Fut { (self.0)(context) }
    }

    /// 🚫️ The (quota+1)th task on one instance is refused with a typed `Fault` — never a
    /// silent drop — while earlier tasks and OTHER instances are unaffected.
    ///
    /// 🔓️ A different instance has its OWN quota accounting, unaffected by 502's exhaustion.
    #[semio_framework_async_macros::async_test]
    async fn spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it() {
        let instance = 502;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        crate::reactor_driver::set_instance_quota(instance, 2).await;

        for label in ["first", "second"] {
            let task = test_task(label, |_ctx| async move { Ok(TaskResolution::Done) });
            semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).unwrap_or_else(|error| panic!("task '{label}' must be admitted under quota 2: {error:?}"));
        }
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 2);

        let third = test_task("third", |_ctx| async move { Ok(TaskResolution::Done) });
        let error = semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, third).expect_err("the 3rd task must be refused — quota is 2, not a silent drop");
        assert_eq!(error.code.0, "plugin.task.quota-exceeded");
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 2, "a refused spawn must not have added a 3rd record");

        let other_instance = 503;
        let other_meta = ActionMeta { actor: "local".into(), instance_id: other_instance, view_state: None };
        let task = test_task("elsewhere", |_ctx| async move { Ok(TaskResolution::Done) });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(other_instance, &other_meta, task).expect("a different instance must not be affected by 502's quota exhaustion");
        crate::reactor_driver::run_until_idle(8).await;
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 0, "the admitted tasks retire their owners before the thread ends");
    }

    /// 🔑️ Spawning a second task under the SAME `(instance, key)` cancels the first — its
    /// future is dropped (never runs to completion, never queues a resume) — and the dedupe
    /// index tracks only the NEW task afterward.
    #[semio_framework_async_macros::async_test]
    async fn key_dedupe_cancels_the_previously_live_task_under_the_same_key() {
        let instance = 504;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let first_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let first_ran_inner = first_ran.clone();
        let first = test_task("first", move |_ctx| async move {
            first_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .keyed("search");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, first).expect("first must be admitted");
        assert!(crate::reactor_driver::task_key_is_live(instance, "search").await);
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 1);

        let second_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let second_ran_inner = second_ran.clone();
        let second = test_task("second", move |_ctx| async move {
            second_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .keyed("search");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, second).expect("second must be admitted, cancelling the first");
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 1, "the SAME key must never have two live tasks at once");

        crate::reactor_driver::run_until_idle(8).await;
        assert!(!first_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled first task must never have run its body");
        assert!(second_ran.load(std::sync::atomic::Ordering::SeqCst), "the surviving second task must have run");
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 0, "the second task completed with TaskResolution::Done — no follow-up, cleaned up");
    }

    /// 🚫️ `Event::InstanceClose` cancellation: every task an instance owns is dropped from the
    /// executor (never runs to completion) and its pending `RequestRegistry` slot is gone too
    /// — no leaked slot — while a DIFFERENT instance's live task/request is untouched.
    ///
    /// parks forever in this test
    ///
    /// 🚫️ `cancel_instance_tasks` alone drops the future (and its parked `RequestFuture`
    /// with it); `RequestRegistry::cancel_instance` is the defense-in-depth sweep `poll`'s
    /// `Event::InstanceClose` arm runs right after — exercised here to prove BOTH steps
    /// together leave no slot behind, matching that call site exactly.
    #[semio_framework_async_macros::async_test]
    async fn instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot() {
        let dying = 505;
        let survivor = 506;
        let dying_meta = ActionMeta { actor: "local".into(), instance_id: dying, view_state: None };
        let survivor_meta = ActionMeta { actor: "local".into(), instance_id: survivor, view_state: None };

        let dying_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let dying_ran_inner = dying_ran.clone();
        let dying_task = test_task("dying", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("never-resolved").await;
            dying_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(dying, &dying_meta, dying_task).expect("dying instance's task must spawn");

        let survivor_task = test_task("survivor", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("also-never-resolved").await;
            Ok(TaskResolution::Done)
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(survivor, &survivor_meta, survivor_task).expect("survivor instance's task must spawn");

        let pending = crate::reactor_driver::run_until_idle(8).await;
        assert!(pending, "both tasks must have parked on their own storage_read");
        assert_eq!(crate::reactor_driver::pending_request_count().await, 2, "one RequestRegistry slot per parked task");

        semio_framework_plugin_reactor_tasks::tasks_task_lane::cancel_instance_tasks(dying);
        let removed = crate::reactor_driver::cancel_instance_registry_requests(dying);
        assert_eq!(removed.await, 0, "cancel_instance_tasks already dropped the task's own RequestFuture — nothing left for the registry sweep to remove");

        assert_eq!(crate::reactor_driver::task_count_for_instance(dying).await, 0, "the dying instance's task record must be gone");
        assert_eq!(crate::reactor_driver::pending_request_count().await, 1, "only the SURVIVOR's request may remain pending");
        assert_eq!(crate::reactor_driver::task_count_for_instance(survivor).await, 1, "the survivor's task must be untouched");

        let still_pending = crate::reactor_driver::run_until_idle(8).await;
        assert!(still_pending, "the survivor's task is still legitimately parked");
        assert!(!dying_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled task must never observe its await resolving, because it never runs again");
    }
