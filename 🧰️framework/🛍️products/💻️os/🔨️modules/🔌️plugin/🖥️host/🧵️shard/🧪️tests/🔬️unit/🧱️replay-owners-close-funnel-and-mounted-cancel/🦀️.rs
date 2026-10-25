// 🧱️ Replay owners, close funnel and mounted cancel: the replay owners dropped from every frontier, the replay failure close funnel and the mounted replay route, seed, generation and worker refusals and cancellation.

//#region 🔖️LanePriorityAndEpochYield
#[semio_framework_async_macros::async_test]
async fn replay_failure_and_actor_loss_enter_one_close_funnel_before_reporting() {
    let _replay_authority = replay_test_authority();
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(74);
    let package = PackageRef { package: PackageId("replay-close-funnel".into()), hash: PackageHash([74; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4_096, 1)).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    let turn = test_job_turn(actor, 24, 0, 0);
    let kind = "close-funnel".to_string();
    let input = vec![1, 2, 3];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let mut seed = MountedReplaySeed::new(actor.0, 24, turn, request, JobPlacement::Inline, kind, input).expect("mounted seed");
    seed.seed.as_mut().expect("fixed seed").kind_pages = JOB_REPLAY_KIND_PAGE_CAPACITY;
    shard.replay_seeds[0] = Some(seed);
    shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
    let error = shard.drive_replay_seed().await.expect_err("capture admission must fault");
    assert!(error.to_string().contains("kind page admission refused"));
    let seed = shard.replay_seeds[0].as_ref().expect("faulted seed remains discoverable");
    assert_eq!(seed.phase, ReplaySeedPhase::Closing);
    assert!(matches!(seed.close_reason, Some(ReplaySeedCloseReason::Fault { stage: "capture-kind", .. })));
    shard.begin_replay_seed_close(0, ReplaySeedCloseReason::Cancelled);
    assert!(matches!(shard.replay_seeds[0].as_ref().expect("first close reason").close_reason, Some(ReplaySeedCloseReason::Fault { stage: "capture-kind", .. })));
    drain_replay_lifecycle(&mut shard, actor.0).await;

    let turn = test_job_turn(actor, 25, 0, 0);
    let kind = "actor-loss".to_string();
    let input = vec![4, 5, 6];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let mut retained = MountedReplaySeed::new(actor.0, 25, turn, request, JobPlacement::Inline, kind, input).expect("retained seed");
    retained.phase = ReplaySeedPhase::Retained;
    shard.replay_seeds[0] = Some(retained);
    shard.running_jobs.insert((actor.0, 25));
    shard.job_turns.insert((actor.0, 25), turn);
    shard.job_authorities.insert((actor.0, 25), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, 25), JobPlacement::Inline);
    shard.replay_seed_refusals[0] = Some(ReplaySpawnRefusal::new(actor.0, 26, b"refused", "kind".into(), vec![7]));
    shard.unregister(actor).await;
    assert!(matches!(shard.replay_seeds[0].as_ref().expect("unregistered seed").close_reason, Some(ReplaySeedCloseReason::ActorLost)));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unregistered seed").phase, ReplaySeedPhase::Closing);
    assert!(!shard.replay_seed_refusals[0].as_ref().expect("unregistered refusal").publish);
    assert!(!shard.running_jobs.contains(&(actor.0, 25)));
    drain_replay_lifecycle(&mut shard, actor.0).await;
    assert!(probe.take_outbound().await.is_empty(), "actor-lost refusal never publishes to a retired actor");
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}

#[semio_framework_async_macros::async_test]
async fn mounted_replay_rejects_wrong_route_seed_generation_and_worker_before_work_and_closes_one_owner_per_sub_eight_ms_opportunity() {
    let _replay_authority = replay_test_authority();
    let actor = ActorId(63);
    let job = 11;
    let turn = test_job_turn(actor, job, 0, 0);
    let kind = "action-bus.fixture".to_string();
    let input = vec![1, 2, 3];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
    let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(LoopbackTransport::default()),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.replay_seeds[0] = Some(MountedReplaySeed::new(actor.0, job, turn, request, JobPlacement::Inline, kind, input).expect("fixed mounted replay seed"));

    let initial_phase = shard.replay_seeds[0].as_ref().expect("mounted seed").phase;
    shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 0, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
    assert!(!shard.drive_replay_seed().await.expect("zero fuel refuses unchanged"));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unchanged seed").phase, initial_phase);
    shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 1, wall_ms: 0, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
    assert!(!shard.drive_replay_seed().await.expect("expired deadline refuses unchanged"));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("unchanged seed").phase, initial_phase);

    let mut wrong_route = request;
    wrong_route.digest[0] ^= 1;
    assert!(shard.validate_replay_request(actor.0, turn, wrong_route).is_err());
    assert!(shard.validate_replay_request(actor.0, JobTurn { operation: JobOperation { seed: turn.operation.seed ^ 1, ..turn.operation }, ..turn }, request).is_err());
    assert!(shard.validate_replay_request(actor.0, JobTurn { operation: JobOperation { generation: turn.operation.generation + 1, ..turn.operation }, ..turn }, request).is_err());
    shard.replay_seeds[0].as_mut().expect("mounted seed").phase = ReplaySeedPhase::Retained;
    assert!(shard.begin_replay_seed(actor.0, turn, request, 0, 0).is_err());
    assert!(shard.begin_replay_seed(actor.0, turn, request, 1, u16::MAX).is_err());

    shard.replay_seeds[0].as_mut().expect("mounted seed").phase = ReplaySeedPhase::CaptureInput;
    shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
    let stale_started = std::time::Instant::now();
    assert!(shard.drive_replay_seed().await.expect("stale actor starts exact close"));
    assert!(stale_started.elapsed() < std::time::Duration::from_millis(8));
    assert_eq!(shard.replay_seeds[0].as_ref().expect("stale seed remains discoverable").phase, ReplaySeedPhase::Closing);
    while shard.replay_seeds[0].is_some() {
        shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
        let started = std::time::Instant::now();
        assert!(shard.drive_replay_seed().await.expect("one close opportunity"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}

#[semio_framework_async_macros::async_test]
async fn mounted_cancel_marks_the_exact_replay_seed_for_incremental_close_before_another_job_step() {
    let _replay_authority = replay_test_authority();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(65);
    let job = 13;
    let package = PackageRef { package: PackageId("mounted-replay-cancel".to_string()), hash: PackageHash([17; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1)).await.expect("mock instantiate");
    let turn = test_job_turn(actor, job, 0, 0);
    let kind = "action-bus.cancel".to_string();
    let input = vec![2, 3, 5, 7];
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let process_pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_bytes_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let (transport, _) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    let mut seed = MountedReplaySeed::new(actor.0, job, turn, request, JobPlacement::Inline, kind, input).expect("fixed mounted replay seed");
    seed.phase = ReplaySeedPhase::Retained;
    shard.replay_seeds[0] = Some(seed);
    shard.running_jobs.insert((actor.0, job));
    shard.job_turns.insert((actor.0, job), turn);
    shard.job_authorities.insert((actor.0, job), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, job), JobPlacement::Inline);
    let mut cancel = MockGuestRuntime::idle_turn(original_retained_turn()).await;
    cancel.effects.push(Effect::CancelJob { job });
    mock.script_turn(actor, cancel).await;

    let budget = fixture_issued_turn(semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance));
    shard.granted_budgets.insert(actor.0, budget);
    let lane = shard.actor_lane(actor.0);
    shard.enqueue_authority(lane, DeferredAuthority::Event { actor: actor.0, event: Some(Event::Wake) }, 1).expect("mounted cancel wake is deferred");
    assert_eq!(shard.pump().await.expect("mounted cancel turn"), 1);
    assert!(!shard.has_lifecycle_retry());
    assert_eq!(mock.cancel_admissions(), 1);
    assert_eq!(mock.step_admissions(), 0, "cancel closes before another guest job step");
    assert_eq!(shard.replay_seeds[0].as_ref().expect("cancelled seed remains discoverable").phase, ReplaySeedPhase::Closing);
    assert!(!shard.running_jobs.contains(&(actor.0, job)));
    while shard.replay_seeds[0].is_some() {
        shard.granted_budgets.insert(actor.0, IssuedShardTurn::whole(semio_framework_actor::Budget { fuel: 1, wall_ms: 1, ..semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance) }, semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, 1).expect("declared replay authority"));
        let started = std::time::Instant::now();
        assert!(shard.drive_replay_seed().await.expect("one cancelled-owner close opportunity"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
    assert_eq!(JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire), process_pages_before);
    assert_eq!(JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire), abi_bytes_before);
}
//#endregion 🔖️LanePriorityAndEpochYield
