// 🔄️ Shard frame round trips: the shard frame round trip macro and its rows, the owned pack round trip of every shard outcome variant and the grant with no envelopes.

//#region 🔖️ShardFrameRoundTrips
/// 🎯️ terra-shard-grants requirement: every `ShardFrame` variant round-trips through the pack
/// codec, INCLUDING the `Envelope` passthrough — the variant the packet brief explicitly says
/// must not be removed as redundant.
macro_rules! shard_frame_round_trip {
    ($name:ident, $value:expr) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            let value: ShardFrame = $value;
            let mut bytes = Vec::new();
            value.pack_encode(&mut bytes).await.expect("declared original frame authority");
            let mut pos = 0usize;
            let decoded = ShardFrame::pack_decode(&bytes, &mut pos).await.expect("pack_decode");
            assert_eq!(pos, bytes.len(), "pack_decode must consume exactly what pack_encode wrote");
            assert_eq!(decoded, value);
        }
    };
}

shard_frame_round_trip!(shard_frame_round_trip_register, ShardFrame::Register { actor: ActorId(7) });
shard_frame_round_trip!(shard_frame_round_trip_unregister, ShardFrame::Unregister { actor: ActorId(9) });
shard_frame_round_trip!(
    shard_frame_round_trip_grant,
    ShardFrame::Grant { retained: original_retained_turn(), retained_policy: original_retained_turn().grant, work_fuel: original_retained_turn().work.fuel, deadline_us: original_retained_turn().work.absolute_deadline_us,         actor: ActorId(11),
        budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive),
        envelopes: vec![Envelope {
            to: ActorId(11),
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: vec![1, 2, 3] },
        }],
    }
);
shard_frame_round_trip!(
    shard_frame_round_trip_envelope_passthrough,
    ShardFrame::Envelope(Envelope { to: ActorId(13), from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: 4 } })
);

#[semio_framework_async_macros::async_test]
async fn shard_outcome_owned_pack_round_trips_every_variant() {
    let operation = test_job_operation(ActorId(11), 1, 0);
    let turn = test_job_turn(ActorId(11), 44, 0, 0);
    let values = vec![
        ShardOutcome::Turn {
            actor: 11,
            result: semio_framework_actor::TurnResult {
                retained_receipt: idle_receipt(),
                ui_patches: vec![1],
                effects: vec![2],
                command_ingress: vec![3],
                cold_pair_ingress: semio_framework_actor::cold_pair::ColdPairIngressStatus::Idle,
                lifecycle_receipt: None,
                ui_patch_receipt: Some(semio_framework_actor::instance_lifetime::ActorUiPatchReceipt {
                    lifetime: semio_framework_actor::instance_lifetime::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 },
                    patch_sequence: 1,
                }),
                next_wake: Some(3),
                status: semio_framework_actor::TurnStatus::MoreWork,
                usage: semio_framework_actor::Usage { fuel: 4, wall_us: 5, memory_bytes: 6 },
            },
            jobs: vec![turn],
        },
        ShardOutcome::Job {
            actor: 11,
            authority: turn,
            request: JobReplayRequest::from_spawn("remodel.reconstruct", b"fixture"),
            placement: JobPlacement::Isolated,
            publication: JobPublication { turn, outcome: JobStepOutcome::PreviewReady { preview: vec![7] } },
        },
        ShardOutcome::Fault { actor: 11, message: "fault".to_string() },
        ShardOutcome::Checkpoint { actor: 11, operation, checkpoint: JobCheckpoint { state: vec![8], applied_progress: 9 } },
        ShardOutcome::Resumed { actor: 11, operation },
        ShardOutcome::Cancelled { actor: 11 },
        ShardOutcome::Preempted { actor: 11 },
    ];
    for value in values {
        let mut bytes = Vec::new();
        value.pack_encode(&mut bytes).await.expect("declared original frame authority");
        assert_eq!(decode_outcome(&bytes).await, value);
    }
}

/// 🎯️ An empty original frame leaves the actor without an issued batch or executable budget.
#[semio_framework_async_macros::async_test]
async fn grant_with_no_envelopes_does_not_issue_a_batch() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(61);
    let package = PackageRef { package: PackageId("grant-empty".to_string()), hash: PackageHash([20u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1, semio_framework_job::default_now_us().unwrap_or(0), 1, 1, 1, 1)).await.expect("mock instantiate");
    let (transport, probe) = LoopbackTransport::paired().await;
    let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    budget.fuel = 123_456;
    let mut bytes = Vec::new();
    ShardFrame::Grant { retained: original_retained_turn(), retained_policy: original_retained_turn().grant, work_fuel: original_retained_turn().work.fuel, deadline_us: original_retained_turn().work.absolute_deadline_us, actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await.expect("declared original frame authority");
    probe.push_inbound(bytes).await;

    let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport),semio_framework_plugin_host_shard::test_identity_issuer()).await;
    shard.register(actor, instance);
    let driven = pump(&mut shard).await.expect("pump");
    assert_eq!(driven, 0, "no envelopes bundled — nothing to drive yet");
    assert!(shard.granted_budget(actor.0).is_none(), "an empty frame supplies no issued batch");
}
//#endregion 🔖️ShardFrameRoundTrips
