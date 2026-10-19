/// 🧷️ Composed checkpoint commit: the reserved spawn job, one ready unit per turn, cancellation between units, child pins and child content publication.

    /// 🧪️ Runs the reserved spawn-job the host would for `admitted` and answers the bytes its
    /// `JobCompleted` carries.
    async fn finish_reserved_spawn_job(admitted: &semio_framework::InvocationResult) -> (u64, Result<Vec<u8>, Fault>) {
        let (job, input) = admitted
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::SpawnJob { job, kind, input, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
                _ => None,
            })
            .expect("a reserved route admits its spawn-job");
        (job, semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job(job, input, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation))
    }

    /// 🧪️ Polls one publication under the same once-paid caller throughout its transaction.
    fn continuation_unit_is_ready_on_first_poll(app:&mut VcsArtifactApp<TestApp,TestMembers>,caller:&mut semio_framework_job::OriginalJobFixtureCaller,grant:semio_framework_value::retained_clone::RetainedCloneGrant){
        let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
        original_publication_turn(app,&mut identity,caller,grant).expect("original continuation unit");
    }

    /// ⚖️ A composed `commitCheckpoint` is a job driven across turns: the host's `JobCompleted` only
    /// queues the commit, every continuation unit completes on its first poll, the commit takes one
    /// unit per composed child plus the parent's own, reports that progress while it runs, and
    /// publishes its result — pins included — only after the last unit.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_checkpoint_commit_is_driven_one_ready_unit_per_turn_with_progress() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let mut publication_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),mounted_policy.preparation,||Some(0),u64::MAX,4096);
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        for child in ["child-1", "child-2"] {
            register_test_child(&mut app, child).await;
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: child.into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        }
        let admitted = PluginApp::handle_action(&mut *app, "commitCheckpoint", Some(dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("admit checkpoint");
        let (job, output) = finish_reserved_spawn_job(&admitted).await;
        assert!(PluginApp::admit_reserved_spawned_job(&mut *app, job, output).expect("JobCompleted queues the commit"));
        assert!(PluginApp::has_runnable_typed_operations(&*app), "a queued commit keeps the actor's turn armed");
        let mut progress = Vec::new();
        let mut units = 0;
        let outcome = loop {
            continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
            units += 1;
            if let Some(outcome) = PluginApp::take_reserved_commit_outcome(&mut *app) {
                break outcome;
            }
            progress.push(PluginApp::reserved_commit_progress(&*app).expect("a running commit reports progress"));
            assert!(units < 64, "the checkpoint commit never finished");
        };
        let result = outcome.expect("checkpoint commit");
        assert_eq!(units, 3, "two composed children plus the parent commit are three units");
        assert_eq!(progress.iter().map(|step| (step.applied, step.total)).collect::<Vec<_>>(), vec![(1, 3), (2, 3)]);
        assert!(progress.iter().all(|step| step.operation == job));
        assert!(result.events.iter().any(|event| event.kind == "history-changed"));
        assert!(PluginApp::reserved_commit_progress(&*app).is_none());
        let checkpoint = app.stores.test_store().await.current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
        let pins = app.stores.test_store().await.envelope().vcs.checkpoints.iter().find(|entry| entry.id == checkpoint).map(|entry| entry.composition_pins.clone()).expect("checkpoint found");
        assert_eq!(pins.iter().map(|pin| pin.child_ref.artifact_id.as_str()).collect::<Vec<_>>(), ["child-1", "child-2"]);
        publication_caller.retire(mounted_policy.close);


        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⚖️ Cancelling the operation between units stops a composed checkpoint before the parent
    /// commits: the outcome is `interactive-job.cancelled`, no parent checkpoint exists, and the app
    /// still closes to terminal emptiness.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_checkpoint_commit_cancels_between_units() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let mut publication_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),mounted_policy.preparation,||Some(0),u64::MAX,4096);
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        for child in ["child-1", "child-2"] {
            register_test_child(&mut app, child).await;
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: child.into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        }
        let admitted = PluginApp::handle_action(&mut *app, "commitCheckpoint", Some(dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("admit checkpoint");
        let (job, output) = finish_reserved_spawn_job(&admitted).await;
        assert!(PluginApp::admit_reserved_spawned_job(&mut *app, job, output).expect("JobCompleted queues the commit"));
        continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
        assert_eq!(PluginApp::reserved_commit_progress(&*app).map(|step| step.applied), Some(1));
        assert!(app.tool_cancellation_handle().cancel_document(semio_framework_plugin_operation::ArtifactDocumentAuthority(meta().instance_id)).expect("cancel the document's operations"));
        continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
        let fault = PluginApp::take_reserved_commit_outcome(&mut *app).expect("a cancelled commit answers").expect_err("cancelled");
        assert_eq!(fault.code.0, "interactive-job.cancelled");
        assert!(app.stores.test_store().await.current_checkpoint_id().is_none(), "a cancelled checkpoint never commits the parent");
        assert!(!PluginApp::has_runnable_typed_operations(&*app));
        publication_caller.retire(mounted_policy.close);


        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📌️ Checkpoint the parent: the cascade must commit the dirty child first, then pin the
    /// child checkpoint that commit produced.
    ///
    /// ⏭️ Move both forward, past the pin.
    ///
    /// ⏮️ Checking the parent out to the pinned checkpoint must drag the child back with it —
    /// otherwise a restored composition silently mixes an old parent with a new child.
    #[semio_framework_async_macros::async_test]
    async fn a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("first composite edit");

        app.dispatch_action("commitCheckpoint", Some(&dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut identity,cx).await.expect("checkpoint");
        let pinned_checkpoint = app.stores.test_store().await.current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
        let pins = app.stores.test_store().await.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == pinned_checkpoint).map(|checkpoint| checkpoint.composition_pins.clone()).expect("checkpoint found");
        assert_eq!(pins.len(), 1, "a composing document's checkpoint must pin its children");
        assert_eq!(pins[0].child_ref.artifact_id, "child-1");

        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 42 }, &meta(), &mut identity,cx).await.expect("second composite edit");
        let live = reads_child_count(&app);
        assert_eq!(live.await, 42);

        app.dispatch_action("checkoutCheckpoint", Some(&dv(serde_json::json!({ "checkpointId": pinned_checkpoint }))), &meta(), &mut identity,cx).await.expect("checkout");
        assert_eq!(reads_child_count(&app).await, 7, "checkout did not cascade to the pinned child");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ The command capture this law holds IS the alias that blocks the retiring root's close —
    /// a real captured root is released when its operation retires, and every assertion above has
    /// already run against it.
    #[semio_framework_async_macros::async_test]
    async fn child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-a").await;
        let admitted = ChildContentView::clone(&app.child_composition.child_content_root);
        let admitted_root = admitted.root.as_ref().expect("published root").clone();
        assert!(std::sync::Arc::ptr_eq(&admitted_root, app.child_composition.child_content_root.root.as_ref().expect("live root")), "command capture retains exactly one immutable root Arc");

        register_test_child(&mut app, "child-b").await;
        let current_root = app.child_composition.child_content_root.root.as_ref().expect("advanced root");
        assert!(!std::sync::Arc::ptr_eq(&admitted_root, current_root), "a child lifecycle event publishes a new root");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-a").is_ok(), "the admitted root remains exact after later publication");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-b").is_err(), "the admitted root never observes a later child");
        assert!(ChildContentView::clone(&app.child_composition.child_content_root).typed_read::<TestSnapshot>("slot", "child-b").is_ok());
        assert!(!app.child_composition.child_content_retirements.is_empty(), "the replaced nonempty root remains under explicit retirement authority");
        drop(admitted);
        drop(admitted_root);
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
