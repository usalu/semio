/// 🔀️ Selected ids, merge vocabulary and checkpoint: the leftover selected ids set, the schema merge vocabulary and the restartable task command requeue after restore.

    /// 🧪 Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-dedupe-2026-09-12.md`): the leftover
    /// `selectedIds` publication is a SET of topology ids — `Select` is event-sourced, so it is
    /// idempotent per id whatever the merge, and however many mirror domains (`vortex`) the flatten
    /// walks. Fixture-driven off `🧫️fixtures/🕹️selection-set.json`, the same file the language-neutral
    /// Node twin (`interactionSelectionSetOracle`) re-derives, so neither side can drift alone.
    /// Fails-before: one plain pick published the id twice, once per mirroring domain.
    #[semio_framework_async_macros::async_test]
    async fn leftover_interaction_view_selected_ids_are_a_set_for_every_merge() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🕹️selection-set.json")).expect("language-neutral selection-set fixture");
        let domain = fixture["domain"].as_str().expect("fixture domain");
        let granularity = fixture["granularity"].as_str().expect("fixture granularity");
        let strings = |value: &Value| -> Vec<String> { value.as_array().expect("fixture id list").iter().map(|id| id.as_str().expect("fixture id").to_string()).collect() };
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert!(!cases.is_empty(), "the selection-set fixture must carry at least one case");
        for case in cases {
            let case_id = case["id"].as_str().expect("case id");
            let seed = strings(&case["seed"]);
            let targets = strings(&case["targets"]);
            let expected = strings(&case["selectedIds"]);
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            if !seed.is_empty() {
                reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": domain, "merge": "replace", "method": "pick" }), granularity, &seed)), &mut identity).await;
            }
            let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": domain, "merge": case["merge"].as_str().expect("case merge"), "method": "pick" }), granularity, &targets)), &mut identity).await;
            let view = settled.output.get("interactionView").expect("leftover InteractionView");
            let published: Vec<String> = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds").iter().map(|id| id.as_str().expect("selected id").to_string()).collect();
            assert_eq!(published, expected, "{case_id}: {}", case["why"].as_str().unwrap_or_default());
            let mut distinct = published.clone();
            distinct.sort();
            distinct.dedup();
            assert_eq!(distinct.len(), published.len(), "{case_id}: selectedIds must name every id at most once, got {published:?}");
            let semio_framework_value::DslValue::Object(locked) = view.get("locked").expect("lock state") else { panic!("{case_id}: locked must be an object") };
            assert_eq!(locked.len(), published.len(), "{case_id}: locked has exactly one key per published id, got {locked:?}");
            let gumball = view.get("gumball").expect("gumball leftover");
            assert_eq!(gumball.get("anchorId").and_then(semio_framework_value::DslValue::as_str), published.first().map(String::as_str), "{case_id}: the gumball anchor is the first published id");
            close_reserved_app(&mut app, mounted_policy,cx);
        }

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    /// 🎯️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-merge-vocabulary-2026-09-12.md`): the
    /// DOMAIN path — the reserved `interactionSelect` job a domain-bound world scene dispatches into —
    /// accepts exactly the five words of `🕹️interaction`'s own `MergeMode` schema enum, and NOTHING
    /// else. Fixture-driven off `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`, the file the Node twin
    /// (`selectionMergeVocabularyOracle`) and the reducer law (`semio-framework`'s
    /// `next_selection_obeys_the_merge_vocabulary_fixture_for_every_mode`) read too.
    ///
    /// Fails-before: the renderer host translated the resolved mode into `add`/`remove`/`toggle`, so a
    /// shift-click / ctrl-click on the generation3d preview faulted `interactionSelect: unknown merge
    /// 'add'` and only the unmodified `replace` pick worked.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_speaks_exactly_the_schema_merge_vocabulary() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🕹️interaction/🧪️tests/🧫️fixtures/🎯️merge-modes.json")).expect("language-neutral merge-modes fixture");
        let words = |value: &Value| -> Vec<String> { value.as_array().expect("fixture word list").iter().map(|word| word.as_str().expect("fixture word").to_string()).collect() };
        let vocabulary = words(&fixture["vocabulary"]);
        let deleted = words(&fixture["deletedWords"]);
        assert!(!vocabulary.is_empty() && !deleted.is_empty(), "the vocabulary fixture must carry both the accepted and the deleted words");
        for word in &vocabulary {
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_targets_args(json!({ "domainId": "items", "merge": word, "method": "pick" }), "item", &["item-1".to_string()])), &mut identity).await;
            assert!(settled.output.get("interactionView").is_some(), "the declared merge '{word}' must settle a leftover InteractionView, not a fault");
            close_reserved_app(&mut app, mounted_policy,cx);
        }
        for word in &deleted {
            let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
            let admitted = app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(interaction_targets_args(json!({ "domainId": "items", "merge": word, "method": "pick" }), "item", &["item-1".to_string()])), &meta(), &mut identity).await.expect("the reserved job is admitted before its args are decoded");
            let fault = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut app, admitted).await.expect_err("a merge word outside the schema vocabulary must fault the reserved job, never fall back to a replace");
            assert!(format!("{fault:?}").contains(&format!("unknown merge '{word}'")), "the deleted word '{word}' must be rejected outright — no adapter, no silent replace; got {fault:?}");
            close_reserved_app(&mut app, mounted_policy,cx);
        }

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📸️ A task's `restart` command survives into the checkpoint pack (not the task itself —
    /// design-abi.md §4) and `restore_now` queues it as an ordinary `Command` resume, ready for
    /// `plugin_resume_task` on the very next turn — exercised end to end against a fresh
    /// `TestApp` instance, exactly like a live task's own resume.
    ///
    /// 🚫️ The in-flight task (and its parked request) belong to the OLD actor incarnation —
    /// never resumed as though the host round-trip were still live (design-abi.md §4).
    /// `restore_now` below is what re-arms it, as a fresh Command resume, not a revival.
    /// The order is the close ladder's own and the completion oracle's
    /// `await-request-retirement-before-observation`: the registry retires its slots FIRST
    /// (`ReactorCloseState::requests_complete` precedes `tasks_complete`), so a task still live
    /// at that moment can only ever observe its request as retired — never as a successful host
    /// response — and `cancel_instance_tasks` then DROPS whatever is left without polling it.
    #[semio_framework_async_macros::async_test]
    async fn checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume() {
        let completion: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let instance = 507;
        semio_framework_plugin_runtime_instances::inst_wire_frames::test_admit_instance_actor(&runtime, instance, "local");
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let restart_command = <TestCommand as ::protocol::OpBinary>::encode_op(&TestCommand::ApplyCountFromTask { value: 7 }).expect("must encode");
        let observed_completion = std::sync::Arc::new(std::sync::Mutex::new(None));

        let task = test_task("checkpointed", {
            let observed_completion = observed_completion.clone();
            move |ctx: TaskCtx| async move {
                let result = ctx.host.storage_read("never-resolved-either").await;
                *observed_completion.lock().unwrap() = Some(result);
                Ok(TaskResolution::Done)
            }
        })
        .restartable(restart_command.clone());
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).expect("must spawn");
        assert!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await);
        let observed_parked_requests = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await;

        let packed = semio_framework_plugin_reactor_core::checkpoint_now(&runtime).await.expect("checkpoint must succeed while the task is in flight");

        assert!(completion["execution"].as_array().unwrap().iter().any(|row| row == "await-request-retirement-before-observation"), "the completion oracle declares this exact order");
        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::cancel_instance_registry_requests(instance).await;
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::poll_instance_tasks_once(instance).await, 1, "the live task observes its retired request on its next poll and finishes");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::cancel_instance_tasks(instance);
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await as u64, completion["checkpoint"]["tasksAfterCancel"].as_u64().unwrap());
        assert_eq!(observed_parked_requests as u64, completion["checkpoint"]["parkedRequests"].as_u64().unwrap(), "checkpoint must observe an actually polled and parked task");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await as u64, completion["checkpoint"]["requestsAfterCancel"].as_u64().unwrap());
        let fault = observed_completion.lock().unwrap().take().expect("the original retained task completed").expect_err("retired request must not become a successful host response");
        assert_eq!(fault.code.0, completion["checkpoint"]["completionFaultCode"].as_str().unwrap());
        assert_eq!(fault.message, completion["checkpoint"]["completionFaultMessage"].as_str().unwrap());

        semio_framework_plugin_reactor_core::restore_now(&runtime, &packed).await.expect("restore must succeed");
        let (resumed_instance, resumed_meta, resumed_input) = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pop_task_resume().await.expect("restore must have queued exactly one resume for the restartable task");
        assert_eq!(resumed_instance, instance);
        assert_eq!(resumed_meta.instance_id, instance);
        let semio_framework_plugin_runtime_ops::ops_typed_operations::TaskResumeInput::Command(bytes) = resumed_input.expect("a restart resume is always Command, never Fault") else { panic!("expected Command") };
        assert_eq!(bytes, restart_command, "restore_now must requeue the EXACT restart bytes the task declared via .restartable(..)");

        let command = <TestCommand as ::protocol::OpBinary>::decode_op(&bytes).expect("the actual restored command must decode");
        assert_eq!(command, TestCommand::ApplyCountFromTask { value: completion["checkpoint"]["restartValue"].as_i64().unwrap() as i32 });
        test_restart_publish_and_close(command, &resumed_meta, &completion).await;
    }
