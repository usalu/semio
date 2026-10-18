// 💰️ Grant budget, frames and budget bridge: the budget a grant executes under, the job step budget, the register and unregister frames and the turn result mapping with the host measured usage.

//#region 🔖️GrantBudgetExecution
// 👶️ host-dedyn: `RecordingRuntime` moved to module level (above `mod tests`) so
// `GuestRuntimes::Recording` (`🖥️host/🦀️.rs`) can name it — see that region's own doc.

/// 🎯️ Headline property test for terra-shard-grants Part B: a `Grant`'s budget — NOT any
/// leftover constant, since `TURN_BUDGET`/`JOB_STEP_BUDGET` are deleted from this file entirely
/// — is what `execute_turn` actually receives. Two DIFFERENT `Grant`s (deliberately distinct
/// `fuel` values) prove the budget travels PER-GRANT, not a single hardcoded number that would
/// happen to match by coincidence.
#[semio_framework_async_macros::async_test]
async fn a_grants_budget_is_what_the_turn_actually_executes_under() {
    let runtime = Arc::new(RecordingRuntime::new().await);
    let actor = ActorId(71);
    let package = PackageRef { package: PackageId("grant-budget".to_string()), hash: PackageHash([21u8; 32]) };
    let compiled = runtime.compile(&package, &[]).await.expect("compile");
    let instance = runtime.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let mut first_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    first_budget.fuel = 111_111;
    let envelope = Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 1,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
    };
    let mut bytes = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), actor, budget: first_budget, envelopes: vec![envelope] }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;
    pump(&mut shard).await.expect("pump 1");
    assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called").fuel, 111_111, "the FIRST Grant's own fuel must reach execute_turn, not a constant");

    let mut second_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background);
    second_budget.fuel = 222_222;
    let envelope2 = Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 2,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
    };
    let mut bytes2 = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), actor, budget: second_budget, envelopes: vec![envelope2] }.pack_encode(&mut bytes2).await.expect("declared original frame authority");
    probe.push_inbound(bytes2).await;
    pump(&mut shard).await.expect("pump 2");
    assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called again").fuel, 222_222, "a DIFFERENT second Grant's fuel must reach execute_turn too — proving it travels per-Grant, not a fixed constant");

    let _ = probe.take_outbound().await;
}

/// 🎯️ Same property for job stepping: `step_job`'s `JobBudget` comes from the SAME actor's last
/// granted budget (point 2 of the brief: "job steps take the owning actor's last granted budget
/// on the Maintenance lane").
#[semio_framework_async_macros::async_test]
async fn job_step_uses_the_owning_actors_last_granted_budget() {
    let _replay_authority = replay_test_authority();
    let runtime = Arc::new(RecordingRuntime::new().await);
    let actor = ActorId(72);
    let package = PackageRef { package: PackageId("grant-job-budget".to_string()), hash: PackageHash([22u8; 32]) };
    let compiled = runtime.compile(&package, &[]).await.expect("compile");
    let instance = runtime.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);

    let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
    budget.fuel = 333_333;
    let mut bytes = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;
    assert_eq!(pump(&mut shard).await.expect("budget grant pump"), 0, "an empty Grant updates authority without driving a guest");
    let turn = test_job_turn(actor, 999, 0, 0);
    let request = JobReplayRequest::from_spawn("grant-budget", &[]);
    shard.running_jobs.insert((actor.0, turn.job));
    shard.job_turns.insert((actor.0, turn.job), turn);
    shard.job_authorities.insert((actor.0, turn.job), JobAuthority { turn, request });
    shard.job_placement.insert((actor.0, turn.job), JobPlacement::Inline);
    let job_bytes = {
        let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Maintenance, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn } };
        let mut out = Vec::new();
        ShardFrame::Envelope(envelope).pack_encode(&mut out).await.expect("declared original frame authority");
        out
    };
    probe.push_inbound(job_bytes).await;
    assert_eq!(pump(&mut shard).await.expect("job step pump"), 1, "the following pump drives the retained job authority");
    assert_eq!(runtime.last_job_budget.lock().unwrap().expect("step_job must have been called").fuel, 333_333, "step_job must run under the SAME actor's last granted budget, not a deleted JOB_STEP_BUDGET constant");
    let _ = probe.take_outbound().await;
}

/// 🎟️ An actor without an actual issued turn has no execution authority.
#[semio_framework_async_macros::async_test]
async fn an_actor_never_granted_authority_preserves_unissued_work() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(73);
    let shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(LoopbackTransport::default()),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    assert!(shard.granted_budget(actor.0).is_none());
}
//#endregion 🔖️GrantBudgetExecution

//#region 🔖️RegisterUnregisterFrames
/// 🛑️ An incoming `ShardFrame::Unregister` must unregister the actor exactly like calling
/// [`ShardLoop::unregister`] directly — real behavior, unlike `Register`'s wire-symmetry-only
/// role (see that variant's own doc).
#[semio_framework_async_macros::async_test]
async fn unregister_frame_drops_the_instance_exactly_like_the_direct_call() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(81);
    let package = PackageRef { package: PackageId("unreg-frame".to_string()), hash: PackageHash([23u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    assert!(shard.is_registered(actor).await);

    let mut bytes = Vec::new();
    ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;
    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 1, "Unregister consumes one bounded authority opportunity");
    assert!(!shard.is_registered(actor).await, "an incoming Unregister frame must drop the instance");
}

/// 📌️ `Register` is decoded without error but has no LOCAL side effect (see its own doc) — this
/// proves `pump` does not choke on it or mistake it for anything else.
#[semio_framework_async_macros::async_test]
async fn register_frame_is_accepted_without_error_and_has_no_local_side_effect() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(82);
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut bytes = Vec::new();
    ShardFrame::Register { actor }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;
    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    let driven = pump(&mut shard).await.expect("pump must not error on a Register frame");
    assert_eq!(driven, 1, "Register consumes one bounded authority opportunity without instantiating locally");
    assert!(!shard.is_registered(actor).await, "Register never instantiates locally — see its own doc");
}
//#endregion 🔖️RegisterUnregisterFrames

//#region 🔖️BudgetBridge
#[semio_framework_async_macros::async_test]
async fn to_actor_turn_result_maps_status_and_carries_host_measured_usage() {
    let kernel_result = TurnResult {
        retained_receipt: idle_receipt(),
        ui_patches: semio_framework::kernel::UiTurnPatches::default(),
        effects: vec![],
        presence: vec![],
        next_wake: Some(42),
        status: semio_framework::kernel::TurnStatus::Faulted(b"trap".to_vec()),
        fuel_used: 999,
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
        lifecycle_receipt: None,
        ui_patch_receipt: None,
    };
    let bridged = to_actor_turn_result(kernel_result, 1, 1234, 5678).await.expect("fixed turn encoding");
    assert_eq!(bridged.next_wake, Some(42));
    assert_eq!(bridged.status, semio_framework_actor::TurnStatus::Faulted { detail: b"trap".to_vec() }, "status must map 1:1, and the actor crate's struct-variant Faulted (Part A) must be what this bridge constructs");
    assert_eq!(bridged.usage.fuel, 999, "usage.fuel comes from the kernel TurnResult's own fuel_used");
    assert_eq!(bridged.usage.wall_us, 1234, "wall_us is host-measured, passed straight through");
    assert_eq!(bridged.usage.memory_bytes, 5678, "memory_bytes is host-measured, passed straight through");
    assert!(bridged.ui_patches.is_empty());
    assert_eq!(bridged.ui_patch_receipt, None);
}

#[semio_framework_async_macros::async_test]
async fn to_actor_turn_result_status_maps_idle_more_work_and_checkpoint_ready() {
    let checkpoint = JobCheckpoint { state: vec![4, 2], applied_progress: 9 };
    for (kernel_status, expected) in [
        (semio_framework::kernel::TurnStatus::Idle, semio_framework_actor::TurnStatus::Idle),
        (semio_framework::kernel::TurnStatus::MoreWork, semio_framework_actor::TurnStatus::MoreWork),
        (semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }, semio_framework_actor::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }),
    ] {
        let kernel_result = TurnResult {
            retained_receipt: idle_receipt(),
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: kernel_status,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        };
        let bridged = to_actor_turn_result(kernel_result, 1, 0, 0).await.expect("fixed turn encoding");
        assert_eq!(bridged.status, expected);
        assert!(bridged.ui_patches.is_empty());
        assert_eq!(bridged.ui_patch_receipt, None);
    }
}

/// 📤️ Repeated validated empty turns use the canonical empty actor representation without
/// reserving transport slots, while one populated owner retains its single-claim lifecycle.
#[semio_framework_async_macros::async_test]
async fn empty_turns_bypass_patch_transport_while_one_populated_owner_claims_once() {
    for session in 1..=semio_framework::kernel::UI_TURN_PATCH_TRANSPORT_SLOTS * 2 {
        let result = TurnResult {
            retained_receipt: idle_receipt(),
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: semio_framework::kernel::TurnStatus::Idle,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        };
        let bridged = to_actor_turn_result(result, session as u64, 0, 0).await.expect("validated empty turn");
        assert!(bridged.ui_patches.is_empty());
        assert_eq!(bridged.ui_patch_receipt, None);
    }

    let mut patches = semio_framework::kernel::UiTurnPatches::default();
    let patch = serde_json::from_value(serde_json::json!({ "surface": "stack-authority", "baseRevision": 0, "revision": 1, "ops": [] })).expect("one patch fixture");
    patches.try_push_ui_patch(patch).expect("one patch owner");
    let receipt = semio_framework::kernel::ActorUiPatchReceipt { lifetime: semio_framework::kernel::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 }, patch_sequence: 1 };
    let result = TurnResult {
        retained_receipt: idle_receipt(),
        ui_patches: patches,
        effects: vec![],
        presence: vec![],
        next_wake: None,
        status: semio_framework::kernel::TurnStatus::Idle,
        fuel_used: 0,
        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
        lifecycle_receipt: None,
        ui_patch_receipt: Some(receipt),
    };
    let session = 800_001;
    let bridged = to_actor_turn_result(result, session, 0, 0).await.expect("populated turn");
    assert_eq!(bridged.ui_patch_receipt, Some(receipt));
    let lease = semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).expect("one exact claim");
    assert!(semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
    let mut owner = lease.take_owner().unwrap_or_else(|_| panic!("one exact owner"));
    assert_eq!(owner.len(), 1);
    while !owner.close_step() {}
    assert!(semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
}
//#endregion 🔖️BudgetBridge
