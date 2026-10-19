/// 👶️ Child emit prepublication and ephemeral lanes: the child emit prepublication close and rejection, the interaction target arguments and both ephemeral lanes.

    #[semio_framework_async_macros::async_test]
    async fn retained_command_child_emit_prepublication_close_and_rejected_handoff_are_bounded() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/🧩️child-prepublication-close.json")).expect("language-neutral child close fixture");
        let expected_ids = fixture["children"].as_array().expect("child close fixture rows").iter().map(|child| child["childId"].as_str().unwrap().to_string()).collect::<Vec<_>>();
        let expected_order = fixture["expectedRetirementOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        let build_emit = || {
            let children = fixture["children"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    let emitted = test_child_emit(child["slot"].as_str().unwrap(), child["childId"].as_str().unwrap(), &[TestMutation::SetCount(SetCount { value: index as i32 + 1 })]);
                    assert!(emitted.ops.iter().all(|op| !op.is_empty()), "real composite mutation owns nonempty encoded bytes");
                    emitted
                })
                .collect();
            Emit { artifact_mutations: vec![TestMutation::SetCount(SetCount { value: 7 })], child_emits: children, ..Default::default() }
        };
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(42), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,crate::job_close_fixture::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
        let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();
        let drive_to_publish = |job:&mut semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>,caller:&mut semio_framework_job::OriginalJobFixtureCaller| {
            for expected_stage in ["preflight", "work"] {
                match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(job,cx))).unwrap() {
                    semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                    _ => panic!("retained child job failed at {expected_stage}"),
                }
            }
        };
        let close_and_observe = |job:&mut semio_framework_plugin_retained_command::ArtifactRetainedCommandJob<TestApp>,caller:&mut semio_framework_job::OriginalJobFixtureCaller| {
            use semio_framework_value::retained_clone::RetainedCloneGrant;
            let initial = job.test_pending_emit_shape().expect("pre-publication emit owner");
            assert_eq!(initial, (2, 1, expected_ids.clone()));
            job.begin_close();
            let quoted = job_close_grant(&*job);
            assert_eq!(caller.turn(RetainedCloneGrant { maximum_items: 0, ..quoted },|cx|job.close_step(cx)).unwrap(), semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "zero item grant preserves the exact emit owner");
            let raw = job_close_grant(&*job);
            let step = caller.turn(raw,|cx|job.close_step(cx)).unwrap();
            assert!(step.progress().fits(raw), "empty reserved raw allocation retires within its exact grant");
            assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "raw allocation retirement preserves the exact emit owner");
            let next = job_close_grant(&*job);
            let demand=semio_framework_value::RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().unwrap(),capacity_bytes:job.next_close_capacity_byte_demand(next.maximum_copy_bytes).unwrap(),release_bytes:job.next_close_release_byte_demand().unwrap(),depth:job.next_close_depth_demand().unwrap()};
            for starved in [
                (demand.copy_bytes != 0).then(|| RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..next }),
                (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..next }),
                (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..next }),
            ]
            .into_iter()
            .flatten()
            {
                assert_eq!(caller.turn(starved,|cx|job.close_step(cx)).unwrap(), semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
                assert_eq!(job.test_pending_emit_shape(), Some(initial.clone()), "a grant short on any axis preserves the exact child payload");
            }
            let mut retired = Vec::new();
            let mut prior_children = expected_ids.len();
            for _ in 0..10_000 {
                let grant = job_close_grant(&*job);
                let step = caller.turn(grant,|cx|job.close_step(cx)).unwrap();
                assert!(step.progress().fits(grant) && grant.maximum_items <= fixture["maximumItems"].as_u64().unwrap() as usize);
                if let Some((children, parents, _)) = job.test_pending_emit_shape() {
                    assert_eq!(parents, 1, "parent lane remains owned until every child is terminal");
                    if children < prior_children {
                        retired.push(fixture["children"][prior_children - 1]["id"].as_str().unwrap());
                        prior_children = children;
                    }
                } else {
                    break;
                }
            }
            assert_eq!(retired, expected_order);
            drive_job_close(job,caller);
            assert!(job.terminal_is_empty());
        };

        let direct_completion = ArtifactToolCompletion::<TestApp>::new();
        let _direct_consumer = direct_completion.clone();
        let (direct_payload,direct_issuer)=test_retained_child_command_payload(direct_completion,build_emit()).await;
        let mut direct=semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(direct_payload);
        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|direct.test_admit_original_source(cx)).unwrap());
        drive_to_publish(&mut direct,&mut caller);
        close_and_observe(&mut direct,&mut caller);direct_issuer.close(&mut caller);crate::job_close_fixture::retire_fixture_owned(_direct_consumer,&mut caller);

        let duplicate_completion = ArtifactToolCompletion::<TestApp>::new();
        let duplicate_consumer = duplicate_completion.clone();
        duplicate_completion.complete(Ok(Emit::default()), EphemeralEmit::default()).expect("prefill completion once");
        let (duplicate_payload,duplicate_issuer)=test_retained_child_command_payload(duplicate_completion,build_emit()).await;
        let mut duplicate=semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(duplicate_payload);
        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|duplicate.test_admit_original_source(cx)).unwrap());
        drive_to_publish(&mut duplicate,&mut caller);
        
        match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut duplicate,cx))).unwrap() {
            semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
            _ => panic!("duplicate completion must reject the handoff"),
        }
        assert!(duplicate.test_pending_emit_shape().is_some(), "duplicate rejection returns the exact emit owner");
        close_and_observe(&mut duplicate,&mut caller);duplicate_issuer.close(&mut caller);
        let (existing, _) = duplicate_consumer.take_emit().expect("prefilled completion remains readable").expect("prefilled completion remains assigned");
        let existing=existing.expect("prefilled completion remains successful");assert!(existing.child_emits.is_empty());crate::job_close_fixture::retire_fixture_owned(existing,&mut caller);crate::job_close_fixture::retire_fixture_owned(duplicate_consumer,&mut caller);

        let busy_completion = ArtifactToolCompletion::<TestApp>::new();
        let busy_consumer = busy_completion.clone();
        let (busy_payload,busy_issuer)=test_retained_child_command_payload(busy_completion.clone(),build_emit()).await;
        let mut busy=semio_framework_plugin_retained_command::ArtifactRetainedCommandJob::new(busy_payload);
        assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|busy.test_admit_original_source(cx)).unwrap());
        drive_to_publish(&mut busy,&mut caller);
        busy_completion.with_busy_test_lock(|| {
            
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut busy,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {},
                _ => panic!("busy completion must reject the handoff"),
            }
        });
        assert!(busy.test_pending_emit_shape().is_some(), "busy rejection returns the exact emit owner");
        close_and_observe(&mut busy,&mut caller);busy_issuer.close(&mut caller);
        assert!(busy_consumer.take_emit().expect("busy completion remains readable").is_none(), "busy completion remains untouched");
        crate::job_close_fixture::retire_fixture_owned(busy_consumer,&mut caller);crate::job_close_fixture::retire_fixture_owned(busy_completion,&mut caller);crate::job_close_fixture::close_fixture_decoder(&mut caller,&mut decoder);caller.retire(crate::job_close_fixture::original_fixture_stock());
    }

    fn interaction_empty_target_args(extra: Value) -> DslValue {
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String("[]".into())));
        }
        object
    }

    /// 🧪️ Same wire shape as {@link interaction_target_args} for a BATCH of targets — the shape a
    /// marquee release (or a pick that resolved several rendered instances) carries.
    fn interaction_targets_args(extra: Value, granularity: &str, ids: &[String]) -> DslValue {
        let targets = serde_json::to_string(&ids.iter().map(|id| InteractionTarget { granularity: granularity.into(), id: id.clone() }).collect::<Vec<_>>()).expect("targets serialize");
        let mut object = semio_framework_value::DslValue::from(&extra);
        if let semio_framework_value::DslValue::Object(entries) = &mut object {
            entries.retain(|(key, _)| key != "targets");
            entries.push(("targets".to_string(), semio_framework_value::DslValue::String(targets)));
        }
        object
    }

    /// 🧾️ Neither ephemeral lane may appear in history: they have no edits, no undo, and no
    /// command-log rows of their own — the document's single edit is the only thing recorded.
    ///
    /// ↩️ Undo rolls back the DOCUMENT; the ephemeral lanes are not restored, because they
    /// were never part of the undoable gesture in the first place.
    #[semio_framework_async_macros::async_test]
    async fn a_command_reaches_both_ephemeral_lanes_without_touching_history() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/🧬️request-context.json")).expect("neutral captured-context fixture");
        let vectors = &fixture["ephemeralPublication"];
        let mut app = contract_app(mounted_policy, &mut identity).await;
        assert_eq!(app.stores.presence_store.generation().await, 0);
        assert_eq!(app.stores.transient_store.generation().await, 0);

        let frame = meta();
        let started = app.test_mount_typed_command(TestCommand::Increment, "increment", &frame, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("mount increment");
        assert!(started.mutations.is_empty(), "mounting does not publish a producer emission");
        assert_eq!(app.stores.presence_store.generation().await, 0, "dispatch did not run an ephemeral prelude");
        assert_eq!(app.stores.transient_store.generation().await, 0, "dispatch did not apply transient work before its worker completion");
        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentEdits": app.stores.test_store().await.envelope().vcs.edits.len() }), vectors["mounted"]);
        settle_contract_app(&mut app,&mut identity,cx).await;

        assert_eq!(app.stores.presence_store.generation().await, 1, "presence lane never received the command's emission");
        assert_eq!(app.stores.transient_store.generation().await, 1, "transient lane never received the command's emission");
        assert_eq!(
            app.ephemeral_snapshot().await,
            EphemeralSnapshot { presence: PublicationPresence { revision: 1 }.encode_pack(), presence_generation: 1, transient_generation: 1, interaction: Vec::new(), tool_run: None, history_edit: None },
            "object-safe channel snapshot must carry the typed presence pack, both generations, and (declaring no interaction domain) empty interaction bytes"
        );

        assert_eq!(app.stores.test_store().await.envelope().vcs.edits.len(), 1, "an ephemeral lane leaked into the document's edit log");

        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentEdits": app.stores.test_store().await.envelope().vcs.edits.len() }), vectors["completed"]);
        app.dispatch_action("undo", None, &meta(), &mut identity,cx).await.expect("undo");
        assert_eq!(app.stores.test_snapshot().await.count, 0);
        assert_eq!(app.stores.presence_store.generation().await, 1, "undo must not rewind presence");
        assert_eq!(app.stores.transient_store.generation().await, 1, "undo must not rewind transient");
        assert_eq!(json!({ "presenceGeneration": app.stores.presence_store.generation().await, "transientGeneration": app.stores.transient_store.generation().await, "documentCount": app.stores.test_snapshot().await.count }), vectors["undone"]);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
