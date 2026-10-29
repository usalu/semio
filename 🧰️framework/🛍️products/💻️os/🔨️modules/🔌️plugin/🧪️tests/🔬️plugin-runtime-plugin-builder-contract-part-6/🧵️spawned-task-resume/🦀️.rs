// 🧵️ Spawned task resume: a task awaiting a real request round trip and the resume that mutates the document.

    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `⚛️reactor::poll`'s real
    // event routing (`Event::Completed`, `run_until_idle`, `drain_task_resumes`) is
    // wasm32-wasip2-only (`wit_bridge`) and cannot run under a native `cargo test`. These tests
    // exercise the SAME underlying primitives `poll` itself drives — `⚛️reactor::spawn_task`,
    // `⚛️reactor::reactor_driver::{run_until_idle, resolve_request, pop_task_resume, ...}`, and
    // `semio_framework_plugin::plugin_runtime::plugin_resume_task` — directly, standing in for the wasm-only
    // glue: `reactor_driver::resolve_request` IS what `Event::Completed` routes to
    // (`RequestRegistry::resolve`), and `pop_task_resume`/`plugin_resume_task` together ARE
    // `drain_task_resumes` minus the frame-routing tail (`route_app_frame`, itself untouched by
    // this packet). An honest, named gap: the wasm-only tail is not exercised natively.

    /// 🌱️ A command spawns a task; the task awaits a REAL `host::storage_read` round trip
    /// (genuinely parked on `RequestRegistry`, not a synchronously-ready future); an injected
    /// completion (`reactor_driver::resolve_request`, standing in for `Event::Completed`)
    /// resolves it; the follow-up `TaskResolution::Command` mutates the store, stamped with
    /// the task's CLONED originating `ActionMeta`.
    ///
    /// 🪪️ This law dispatches under its OWN instance, not the fixture's default: a typed command
    /// is refused with `interactive-job.live-instance` unless the wrapper's mounted live instance
    /// is exactly `meta.instance_id`, and `spawn_task` is keyed off that same id.
    ///
    /// 🪜️ A migrated verb's `dispatch_typed` only ADMITS: the emit is owned by the mounted
    /// operation, and every lane of it — the task lane included — is spent by the bounded
    /// publication ladder, one lane per unit. Asserting the spawn before settling would assert
    /// that a migrated command never spawns.
    ///
    /// 🪪️ `spawn_task` is keyed off `meta.instance_id` (the publication ladder's own captured
    /// `mounted.meta`, i.e. `spawn_meta` above, which shares `instance`'s value by construction).
    ///
    /// ▶️ First poll: the task runs up to its `.await` on `host.storage_read(..)` and parks —
    /// genuinely pending, not synchronously resolved.
    ///
    /// ✅️ Inject the "completion" (`⚛️reactor::poll`'s `Event::Completed` arm calls the exact
    /// same `RequestRegistry::resolve`) — request id 1 is `storage_read`'s, the first (and
    /// only) request this test has allocated.
    ///
    /// 🔀️ Re-enter through the SAME typed-command path a live command would — this IS
    /// `plugin_runtime::plugin_resume_task`'s `TaskResumeInput::Command` arm, exercised
    /// directly rather than through `crate::reactor::drain_task_resumes` (wasm-only).
    ///
    /// 🪜️ The follow-up re-enters the APPLYING lane: `PluginApp::queue_task_resume_command` only
    /// QUEUES it on the instance, which delivers it through its original typed source one funded
    /// unit per publication turn, so like every migrated dispatch it ADMITS there and the document
    /// advances when its publication ladder walks. Settle it through the same runtime cell the
    /// host's continuation drives, then read what it actually did.
    ///
    /// 🧾️ And it landed as the app's own verb in the command log, applied — a PREVIEW
    /// (`handle_command_frame`, the agent prepare lane this resume used to take) records
    /// nothing at all and applies nothing.
    #[semio_framework_async_macros::async_test]
    async fn a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let instance = 501;
        let spawn_meta = ActionMeta { actor: "alice".into(), instance_id: instance, view_state: None };
        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        semio_framework_plugin_reactor_app::PluginApp::bind_instance_id(&mut app, instance).await;

        let result = semio_framework_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut app, &TestCommand::SpawnCountTask, &spawn_meta, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity, cx).await.expect("dispatching SpawnCountTask must succeed");
        assert!(result.mutations.is_empty(), "SpawnCountTask itself must emit no document mutation — only the LATER resume does");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app, instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the spawning operation's publication settles");
        assert_eq!(
            receipt.lanes,
            vec![TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal],
            "the task lane spends its own publication unit and mints NO host page of its own: the operation's pages are exactly the ones its emit's remaining lanes owe"
        );
        assert_eq!(crate::reactor_driver::task_count_for_instance(instance).await, 1, "the publication ladder's task lane must have spawned exactly one task");

        let pending = crate::reactor_driver::run_until_idle(8).await;
        assert!(pending, "the task must be parked on a real RequestRegistry await, not finished synchronously");
        assert_eq!(crate::reactor_driver::pending_request_count().await, 1, "storage_read must have allocated exactly one RequestRegistry slot");
        assert!(crate::reactor_driver::pop_task_resume().await.is_none(), "nothing can have resolved yet — the task is still parked");

        crate::reactor_driver::resolve_request(1, Ok(42i32.to_le_bytes().to_vec())).await;
        let pending = crate::reactor_driver::run_until_idle(8).await;
        assert!(!pending, "the task must run to completion once its await resolves");
        assert_eq!(crate::reactor_driver::task_count_for_instance(spawn_meta.instance_id).await, 0, "TASK_RECORDS must be cleaned up the moment the task's future completes");

        let (resumed_instance, resumed_meta, resumed_input) = crate::reactor_driver::pop_task_resume().await.expect("the completed task must have queued exactly one resume");
        assert_eq!(resumed_instance, spawn_meta.instance_id);
        assert_eq!(resumed_meta.actor.as_str(), "alice", "the follow-up must carry the task's ORIGINATING actor, not whatever is 'current' at resume time");
        assert_eq!(resumed_meta.instance_id, spawn_meta.instance_id);
        let semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(command_bytes) = resumed_input.expect("the task resolved Ok, not with a Fault") else {
            panic!("SpawnCountTask's task resolves TaskResolution::Command, not ::Emit");
        };
        let decoded_command = <TestCommand as ::protocol::OpBinary>::decode_op(&command_bytes).expect("must decode back to a TestCommand");
        assert_eq!(decoded_command, TestCommand::ApplyCountFromTask { value: 42 }, "the resolved command must carry the value the injected completion delivered");

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app), surface_contexts: Default::default() }).await;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, resumed_instance, "local");
        let output = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_resume_task(&runtime, resumed_instance, &resumed_meta, semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(command_bytes), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await;
        assert!(output.frames.is_empty(), "queueing a resumed Command frames nothing: its receipt follows the instance's own delivery: {:?}", output.frames.len());

        let cell = semio_framework_plugin_runtime_ops::ops_wire_limits::runtime_instance_cell(&runtime, resumed_instance).expect("the resumed instance stays mounted");
        let mut active = cell.instance.try_lock().expect("no other owner holds the resumed instance");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut active.app, resumed_instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the resumed follow-up publishes");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Artifact), "the follow-up must have published on the ARTIFACT lane — it applies, it does not preview: {:?}", receipt.lanes);
        assert_eq!(receipt.completions, 1, "the follow-up is exactly one terminated operation");
        assert!(matches!(receipt.resume_outcomes.as_slice(), [Ok(_)]), "the delivered follow-up answers once with its mounted receipt: {:?}", receipt.resume_outcomes.len());

        let history = semio_framework_plugin_reactor_app::PluginApp::history_snapshot(&mut active.app).await.expect("history after the resumed follow-up");
        let row = history.upserts.iter().find(|entry| entry.action_id == "applyCountFromTask").unwrap_or_else(|| panic!("the follow-up dispatch is recorded under its own verb: {:?}", history.upserts.iter().map(|entry| entry.action_id.clone()).collect::<Vec<_>>()));
        assert!(row.applied, "the follow-up is an APPLIED edit, not a staged preview");
        assert!(row.op_lines.iter().any(|line| line.contains("42")), "the applied edit carries the value the injected completion delivered: {:?}", row.op_lines);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
}

    /// 🔁️ Fixture setup shared by the task-resume delivery laws: the explicit mounted policy and the original identity authority.
    fn resume_policy() -> semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy()
    }

    fn resume_meta(instance: u32) -> ActionMeta {
        ActionMeta { actor: "alice".into(), instance_id: instance, view_state: None }
    }

    fn resume_bytes(value: i32) -> Vec<u8> {
        <TestCommand as ::protocol::OpBinary>::encode_op(&TestCommand::ApplyCountFromTask { value }).expect("the follow-up command encodes")
    }

    /// 🏃️ A queued follow-up is delivered by the instance across several funded publication units, never in the queueing call, and answers exactly once with its mounted receipt.
    #[semio_framework_async_macros::async_test]
    async fn a_queued_task_resume_is_delivered_across_funded_turns_and_answers_once() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_policy = resume_policy();
        let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
        let instance = 502;
        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        semio_framework_plugin_reactor_app::PluginApp::bind_instance_id(&mut app, instance).await;

        semio_framework_plugin_reactor_app::PluginApp::queue_task_resume_command(&mut app, resume_bytes(5), &resume_meta(instance)).expect("the follow-up queues");
        assert!(semio_framework_plugin_reactor_app::PluginApp::has_runnable_typed_operations(&app), "a queued follow-up keeps the actor's turn armed");
        assert_eq!(app.stores.test_snapshot().await.count, 0, "queueing applies nothing");

        let mut units = 0;
        let outcome = loop {
            assert!(units < semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(), "the follow-up never settled");
            semio_framework_plugin_reactor_app::PluginApp::advance_typed_operation_publication(&mut app, &mut identity, cx).await.expect("one funded publication unit");
            units += 1;
            if let Some(outcome) = semio_framework_plugin_reactor_app::PluginApp::take_task_resume_outcome(&mut app) { break outcome; }
        };
        assert!(units > 1, "the delivery spans several funded units, got {units}");
        outcome.expect("the follow-up mounted");
        assert!(semio_framework_plugin_reactor_app::PluginApp::take_task_resume_outcome(&mut app).is_none(), "the follow-up answers exactly once");

        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app, instance, mounted_policy, &mut identity, cx).await.expect("the mounted follow-up publishes");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Artifact), "{:?}", receipt.lanes);
        assert_eq!(app.stores.test_snapshot().await.count, 5, "the follow-up applied exactly once");
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy, cx).expect("original contract fixture paid terminal close");
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🛑️ A follow-up the app cannot deliver answers its task with a fault frame's fault instead of a lost result or a failed turn, and the lane keeps serving the follow-ups behind it.
    #[semio_framework_async_macros::async_test]
    async fn a_refused_task_resume_answers_with_a_fault_and_the_next_one_still_delivers() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_policy = resume_policy();
        let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
        let instance = 503;
        let mut app = VcsArtifactApp::<TestApp<false, 1>>::new(TestApp::<false, 1>::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.expect("the reference snapshot births");
        semio_framework_plugin_reactor_app::PluginApp::bind_instance_id(&mut app, instance).await;

        for value in [1, 2] {
            semio_framework_plugin_reactor_app::PluginApp::queue_task_resume_command(&mut app, resume_bytes(value), &resume_meta(instance)).expect("the follow-up queues");
        }
        let mut faults = Vec::new();
        for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns() {
            if !semio_framework_plugin_reactor_app::PluginApp::has_pending_typed_operations(&app) { break; }
            semio_framework_plugin_reactor_app::PluginApp::advance_typed_operation_publication(&mut app, &mut identity, cx).await.expect("a refused delivery is an answer, not a failed turn");
            if let Some(outcome) = semio_framework_plugin_reactor_app::PluginApp::take_task_resume_outcome(&mut app) { faults.push(outcome.expect_err("the registry-less app refuses every follow-up")); }
        }
        assert_eq!(faults.len(), 2, "every queued follow-up is answered, in order");
        assert!(faults.iter().all(|fault| fault.code.0 == "interactive-job.task-resume-delivery"), "{:?}", faults.iter().map(|fault| fault.code.0.clone()).collect::<Vec<_>>());
        assert!(!app.original_typed_command.pending(), "the failed delivery returned the typed source through paid closure");
        assert_eq!(app.stores.test_snapshot().await.count, 0);
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy, cx).expect("original contract fixture paid terminal close");
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪦️ Cancelling after 0..k funded units retires the queue: the typed source returns every owner through paid closure and nothing is applied or answered.
    #[semio_framework_async_macros::async_test]
    async fn a_cancelled_task_resume_queue_returns_every_owner_after_any_number_of_turns() {
        for cut in 0..8 {
            let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
            let mounted_policy = resume_policy();
            let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
            let instance = 504;
            let mut app = contract_app_raw(mounted_policy, &mut identity).await;
            semio_framework_plugin_reactor_app::PluginApp::bind_instance_id(&mut app, instance).await;
            for value in [1, 2] {
                semio_framework_plugin_reactor_app::PluginApp::queue_task_resume_command(&mut app, resume_bytes(value), &resume_meta(instance)).expect("the follow-up queues");
            }
            for _ in 0..cut {
                semio_framework_plugin_reactor_app::PluginApp::advance_typed_operation_publication(&mut app, &mut identity, cx).await.expect("one funded publication unit");
                drop(semio_framework_plugin_reactor_app::PluginApp::take_task_resume_outcome(&mut app));
            }
            cx.original_cancel_token().cancel_now();
            for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns() {
                if app.host_events.resumes.is_empty() && !app.original_typed_command.pending() { break; }
                semio_framework_plugin_reactor_app::PluginApp::advance_typed_operation_publication(&mut app, &mut identity, cx).await.expect("a cancelled unit retires, it does not fail");
            }
            assert!(app.host_events.resumes.is_empty() && !app.original_typed_command.pending(), "cancel after {cut} units left the queue or the typed source owned");
            assert!(semio_framework_plugin_reactor_app::PluginApp::take_task_resume_outcome(&mut app).is_none(), "cancel after {cut} units answered a cancelled follow-up");
            }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
        }
    }
