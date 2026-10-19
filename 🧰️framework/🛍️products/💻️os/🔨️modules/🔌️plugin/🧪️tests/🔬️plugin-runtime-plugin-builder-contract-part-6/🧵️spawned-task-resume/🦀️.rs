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
    /// 🪜️ The follow-up re-enters the APPLYING lane (`PluginApp::resume_task_command` →
    /// `dispatch_typed`), so like every migrated dispatch it ADMITS here and the document
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

        let result = app.dispatch_typed(TestCommand::SpawnCountTask, &spawn_meta, &mut identity).await.expect("dispatching SpawnCountTask must succeed");
        assert!(result.mutations.is_empty(), "SpawnCountTask itself must emit no document mutation — only the LATER resume does");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app, instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the spawning operation's publication settles");
        assert_eq!(
            receipt.lanes,
            vec![TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal],
            "the task lane spends its own publication unit and mints NO host page of its own: the operation's pages are exactly the ones its emit's remaining lanes owe"
        );
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 1, "the publication ladder's task lane must have spawned exactly one task");

        let pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(pending, "the task must be parked on a real RequestRegistry await, not finished synchronously");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await, 1, "storage_read must have allocated exactly one RequestRegistry slot");
        assert!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.is_none(), "nothing can have resolved yet — the task is still parked");

        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::resolve_request(1, Ok(42i32.to_le_bytes().to_vec())).await;
        let pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(!pending, "the task must run to completion once its await resolves");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(spawn_meta.instance_id).await, 0, "TASK_RECORDS must be cleaned up the moment the task's future completes");

        let (resumed_instance, resumed_meta, resumed_input) = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.expect("the completed task must have queued exactly one resume");
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
        let output = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_resume_task(&runtime, resumed_instance, &resumed_meta, semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(command_bytes), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        assert_eq!(output.frames.len(), 1, "a successful resume must frame exactly one AppFrame::Emit");
        let frame = protocol::decode_app_frame(&output.frames[0]).await.expect("must decode back to an AppFrame");
        assert!(
            matches!(frame, protocol::AppFrame::Emit { .. }),
            "a resumed Command follow-up must frame as AppFrame::Emit — never a fault: {frame:?}"
        );

        let cell = semio_framework_plugin_runtime_ops::ops_wire_limits::runtime_instance_cell(&runtime, resumed_instance).expect("the resumed instance stays mounted");
        let mut active = cell.instance.try_lock().expect("no other owner holds the resumed instance");
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut active.app, resumed_instance, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the resumed follow-up publishes");
        assert!(receipt.lanes.contains(&TypedOperationResultLane::Artifact), "the follow-up must have published on the ARTIFACT lane — it applies, it does not preview: {:?}", receipt.lanes);
        assert_eq!(receipt.completions, 1, "the follow-up is exactly one terminated operation");

        let history = semio_framework_plugin_reactor_app::PluginApp::history_snapshot(&mut active.app).await.expect("history after the resumed follow-up");
        let row = history.upserts.iter().find(|entry| entry.action_id == "applyCountFromTask").unwrap_or_else(|| panic!("the follow-up dispatch is recorded under its own verb: {:?}", history.upserts.iter().map(|entry| entry.action_id.clone()).collect::<Vec<_>>()));
        assert!(row.applied, "the follow-up is an APPLIED edit, not a staged preview");
        assert!(row.op_lines.iter().any(|line| line.contains("42")), "the applied edit carries the value the injected completion delivered: {:?}", row.op_lines);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
