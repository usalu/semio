/// 📍️ Retained operation, close and checkpoint: command admission continuation, the one segment close step, the shared retained command checkpoint and retained presence.

    /// ⚖️ LAW: a staged operation costs the host ONE turn per acknowledgeable result page, not one
    /// per publication unit. The host has to come back for a result page because the page is only
    /// released by its own ACK; every other unit is internal to the guest, and answering `MoreWork`
    /// after each of them cost this fixture 48 turns before ticket 26/09/02 wave B24.
    /// 🧮️ A turn the operation was not even schedulable on — `contended` without `runnable`, the
    /// shared worker pool busy with somebody else — is the machine's pacing, not the continuation's,
    /// and is not counted: on a loaded machine it drew one such turn in roughly one run in five
    /// (measured 2026-09-21, trace `[(false, true, true), (true, false, true), …]`) while every
    /// runnable turn still carried its exact receipt. The ceiling constant is untouched.
    #[semio_framework_async_macros::async_test]
    async fn retained_operation_continues_after_command_admission_until_publication_and_retirement() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        let id = fixture["wire"]["receiver"].as_u64().unwrap() as u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let value = fixture["command"]["value"].as_i64().unwrap() as i32;
        let command = TestCommand::CompositeEdit { slot: String::new(), child_id: fixture["command"]["target"].as_str().unwrap().into(), child_value: value };
        semio_framework_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut app, &command, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity, cx).await.unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut terminal = false;
        let mut receipts = 0;
        let mut spent = 0;
        let mut turns: Vec<(bool, bool, bool)> = Vec::new();
        for turn in 0..fixture["command"]["maximumTurns"].as_u64().unwrap() {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime,cx).unwrap();
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(reactor_native_budget(cx),cx), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            let more = scan.runnable || scan.contended;
            spent += u64::from(scan.runnable || !scan.contended);
            turns.push((scan.runnable, scan.contended, output.is_some()));
            if let Some((receiver, _output)) = output {
                assert_eq!(receiver, id);
            }
            {
                for page in native_pages {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                    terminal |= page.lane == TypedOperationResultLane::Terminal;
                    receipts += 1;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.close(issued.unwrap_or(mounted_policy.close))).await.unwrap();
                }
            }
            if !more {
                assert!(terminal, "the actor must remain runnable until its terminal result");
                let _ = turn;
                break;
            }
            std::thread::yield_now();
        }
        assert!(
            spent <= receipts + TYPED_OPERATION_CONTINUATION_SLACK,
            "a {receipts}-receipt operation spent {spent} continuation turns (ceiling {}) — the continuation is pacing one publication unit per host round trip; turns (runnable, contended, produced): {turns:?}",
            receipts + TYPED_OPERATION_CONTINUATION_SLACK
        );
        let active = cell.instance.lock().unwrap();
        assert!(terminal);
        assert!(!active.app.has_pending_typed_operations());
        assert_eq!(active.app.snapshot().unwrap().count, value);
        drop(active);
        let retired = std::sync::Arc::downgrade(&cell);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            if let Err(error) = semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx) {
                let entries = runtime.close_quarantine.borrow();
                let state = &entries.get(id).unwrap().state;
                let pump = state.pump.lock().unwrap();
                let detail = state.last_fault.lock().unwrap();
                panic!(
                    "{error:?}: elapsed={} stalled={} terminal={} complete={} blocked={} faulted={} pending={:?} origin={} detail={}",
                    state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst),
                    state.stalled_steps.load(std::sync::atomic::Ordering::SeqCst),
                    pump.terminal,
                    pump.complete,
                    pump.blocked,
                    pump.faulted,
                    pump.pending_status,
                    state.last_fault_origin.load(std::sync::atomic::Ordering::SeqCst),
                    String::from_utf8_lossy(&detail[..])
                );
            }
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(runtime.close_quarantine.borrow().get(id).is_none());
        assert!(retired.upgrade().is_none());
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    /// 🪜️ Park the ladder exactly ON the segment stage without touching it. Every stage before it
    /// is bookkeeping that releases no bytes, and the segment stage alone refuses a grant under one
    /// chunk (`maximum_bytes < ARTIFACT_OUTPUT_CHUNK_BYTES`), so a sub-chunk budget walks the ladder
    /// to that stage and then stands still there — which is itself the bounded-grant refusal.
    #[semio_framework_async_macros::async_test]
    async fn app_close_step_drains_at_most_one_segment_and_one_chunk_budget() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.close.close_started = true;
        app.close.close_cancellation_cursor = TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS;
        app.close.close_media_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        app.close.close_media_cleanup_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES * 2);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.push(vec![2; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert!(app.media.segmented_downloads.insert(37, ArtifactDownloadOutput::new("close.bin", "application/octet-stream", None, chunks.clone()).expect("sealed close output")).is_ok());
        let mut parked = false;
        for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
            let demand = app.close_retirement_demands(ARTIFACT_OUTPUT_CHUNK_BYTES).expect("quoted pre-segment close demand");
            let sub_chunk = store::RetainedCloneGrant { maximum_release_bytes: demand.release_bytes.min(ARTIFACT_OUTPUT_CHUNK_BYTES - 1), ..fixture_close_grant(demand) };
            match { let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,sub_chunk,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty()))).expect("original bounded pre-segment close slice"); let _ = app.take_constructor_receipt(); step } {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress == store::RetainedCloneProgress::default() => {
                    assert_eq!(demand.release_bytes, ARTIFACT_OUTPUT_CHUNK_BYTES, "the ladder parked before its segmented-download rung");
                    parked = true;
                    break;
                }
                step => assert_eq!(step.progress().map(|progress| progress.released_bytes), Some(0), "a pre-segment close stage released bytes under a sub-chunk grant: {step:?}"),
            }
        }
        assert!(parked, "the close ladder never reached its segmented-download stage");
        assert_eq!(chunks.chunks_remaining(), 2, "walking the ladder to the segment stage released no chunk");
        assert_eq!(app_close_turn(&mut *app,cx).expect("first close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.chunks_remaining(), 1);
        assert_eq!(app_close_turn(&mut *app,cx).expect("second close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.chunks_remaining(), 0);
        assert_eq!(app_close_turn(&mut *app,cx).expect("terminal close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert!(!app.media.segmented_downloads.contains(37));


        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close() {
        let bus = ActionBus::new();
        bus.register(TestRetainedCommandFactory::new()).expect("test retained command factory");
        let command = TestCommand::SetLabel { value: "wire".into() };
        let wire = <TestCommand as protocol::OpBinary>::encode_op(&command).expect("test retained command wire");
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(41), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,crate::job_close_fixture::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
        let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();
        let original_completion = ArtifactToolCompletion::<TestApp>::new();
        let original_consumer = original_completion.clone();
        let (original_payload,original_issuer) = test_retained_command_payload(original_completion).await;
        let (original_admission, original_input) = test_retained_wire_input(&bus, &wire);
        let mut original=crate::job_close_fixture::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),original_payload,Some(original_input),None);
        let mut checkpoint_bytes = None;
        for _ in 0..64 {
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut original,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint { state: bytes, .. } => {
                    if bytes.get(4) == Some(&3) && bytes.get(5) == Some(&1) && bytes.get(48) == Some(&1) {
                        checkpoint_bytes = Some(bytes);
                    }
                    if checkpoint_bytes.is_some() {
                        break;
                    }
                }
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Yield | semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Idle => {}
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("initial retained command cancelled"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete { .. } => panic!("initial retained command completed before its work checkpoint"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {
                    panic!("initial retained command faulted");
                }
            }
        }
        let checkpoint_bytes = checkpoint_bytes.expect("work checkpoint after one custom cursor step");
        original.begin_close();
        drive_job_close(&mut original,&mut caller);
        assert!(original.terminal_is_empty(), "interrupted original retires every retained owner incrementally");
        assert!(original_consumer.take_emit().expect("original completion cell").is_none(), "interrupted original never publishes a result");

        original_issuer.close(&mut caller);crate::job_close_fixture::retire_fixture_owned(original_consumer,&mut caller);
        TEST_RETAINED_COMMAND_STEP_CALLS.store(0, std::sync::atomic::Ordering::SeqCst);
        let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
        let resumed_consumer = resumed_completion.clone();
        let (resumed_payload,resumed_issuer) = test_retained_command_payload(resumed_completion).await;
        let (resumed_admission, resumed_input) = test_retained_wire_input(&bus, &wire);
        let (_, resumed_checkpoint) = test_retained_wire_input(&bus, &checkpoint_bytes);
        let mut resumed=crate::job_close_fixture::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),resumed_payload,Some(resumed_input),Some(resumed_checkpoint));
        let mut completed = false;
        for _ in 0..96 {
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut resumed,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint { .. } => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Yield | semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Idle => {}
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete { .. } => {
                    completed = true;
                    break;
                }
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("resumed retained command cancelled"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {
                    panic!("resumed retained command faulted");
                }
            }
        }
        assert!(completed, "resumed retained command reaches its commit boundary");
        assert_eq!(TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2, "resume continues after cursor one instead of replaying completed custom work");
        let (result, _) = resumed_consumer.take_emit().expect("resumed completion cell").expect("resumed completion value");
        let result=result.expect("resumed command emit");assert_eq!(result.artifact_mutations,vec![TestMutation::SetLabel(SetLabel{value:"resumed".into()})]);crate::job_close_fixture::retire_fixture_owned(result,&mut caller);
        resumed.begin_close();
        drive_job_close(&mut resumed,&mut caller);
        assert!(resumed.terminal_is_empty(), "completed resume retires every retained owner incrementally");

        resumed_issuer.close(&mut caller);crate::job_close_fixture::retire_fixture_owned(resumed_consumer,&mut caller);
        let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
        let cancelled_consumer = cancelled_completion.clone();
        let (cancelled_payload,cancelled_issuer) = test_retained_command_payload(cancelled_completion).await;
        let (cancelled_admission, cancelled_input) = test_retained_wire_input(&bus, &wire);
        let mut cancelled=crate::job_close_fixture::admit_registered_fixture(&bus,&mut caller,&mut decoder,operation,action_bus::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER,TEST_RETAINED_COMMAND_TOOL),TEST_RETAINED_COMMAND_SCHEMA.into(),cancelled_payload,Some(cancelled_input),None);
        caller.cancel().cancel_now();
        assert!(matches!(caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        cancelled.begin_close();
        drive_job_close(&mut cancelled,&mut caller);
        assert!(cancelled.terminal_is_empty(), "cancelled retained command retires every owner incrementally");
        assert!(cancelled_consumer.take_emit().expect("cancelled completion cell").is_none(), "cancelled retained command never publishes a result");
        cancelled_issuer.close(&mut caller);crate::job_close_fixture::retire_fixture_owned(cancelled_consumer,&mut caller);crate::job_close_fixture::close_fixture_decoder(&mut caller,&mut decoder);caller.retire(crate::job_close_fixture::original_fixture_stock());
    }

    /// 👥️ Contract-freeze §C7.6: `adopt_presence` (1) adopts an app-typed `presence_pack` into
    /// `presence_store` ONLY when one is present, (2) unconditionally upserts `color`/`surface`/
    /// `interaction` into `peer_presence` for every peer in the roster, and (3) treats the roster
    /// as the single source of truth — a peer absent from a later call is dropped from BOTH maps.
    ///
    /// 👋 bob leaves the roster — a second call carrying only alice must drop bob from BOTH maps.
    #[semio_framework_async_macros::async_test]
    async fn retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let alice = sample_presence_peer("user:alice#s1", Some(3), true);
        let bob = sample_presence_peer("user:bob#s1", Some(5), false);

        assert!(publish_presence_roster(&mut app, 1, Some(9), &[alice.clone(), bob], 1000,cx).await.fault.is_none());
        assert_eq!(app.roster.own_color, Some(9));
        let typed_root = app.stores.presence_store.peers_root();
        assert_eq!(typed_root.peers().count(), 1, "only the peer carrying a presence_pack adopts into presence_store");
        assert_eq!(typed_root.peers().next().unwrap().0, "user:alice#s1");
        assert_eq!(app.roster.peer_presence.len(), 2, "both peers upsert into peer_presence regardless of presence_pack");
        assert_eq!(app.roster.peer_presence.get("user:alice#s1").unwrap().color, Some(3));
        assert_eq!(app.roster.peer_presence.get("user:bob#s1").unwrap().color, Some(5));
        drop(typed_root);

        assert!(publish_presence_roster(&mut app, 2, Some(9), &[alice], 2000,cx).await.fault.is_none());
        assert_eq!(app.stores.presence_store.peers_root().peers().count(), 1);
        assert_eq!(app.roster.peer_presence.len(), 1);
        assert!(app.roster.peer_presence.contains_key("user:alice#s1"));
        assert!(!app.roster.peer_presence.contains_key("user:bob#s1"), "an actor absent from the roster must be dropped, not left stale");
        drain_and_close_fixture(&mut app, mounted_policy,cx);


        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
