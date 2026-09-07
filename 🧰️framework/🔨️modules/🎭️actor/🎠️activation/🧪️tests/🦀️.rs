use super::*;

fn request() -> KernelActivationRequest {
    let package = PackageId("reserved-map".into());
    KernelActivationRequest { package: package.clone(), plugin_ordinal: 7, kind: ActorKind::PluginApp { plugin: package, app_id: "map".into(), instance_id: 1 }, lane: Lane::Interactive, window: None, event: ActivationEvent::Manual }
}

fn binding(reservation: &KernelActivationReservation) -> ActorShardKey {
    ActorShardKey { actor: reservation.actor(), shard: reservation.shard(), registration: NonZeroU64::new(17).unwrap() }
}

#[semio_framework_async_macros::async_test]
async fn neutral_reservation_traces_gate_dispatch_until_exact_binding() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap();
    for row in fixture["traces"].as_array().unwrap() {
        let mut kernel = Kernel::new(ShardKind::Native, 1, 0, 4).await;
        let mut reservation = None;
        let mut actor = None;
        let mut grants = 0;
        for event in row["events"].as_array().unwrap() {
            match event.as_str().unwrap() {
                "reserve" => {
                    let owner = kernel.reserve_activation(request()).await.unwrap();
                    actor = Some(owner.actor());
                    assert!(!kernel.scheduler.actors[&owner.actor()].active);
                    reservation = Some(owner);
                }
                "submit" => {
                    let envelope = Envelope { to: actor.unwrap(), from: Origin::Kernel, lane: Lane::Interactive, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1] } };
                    assert!(matches!(kernel.submit(&envelope).await, Backpressure::Accept));
                }
                "tick" => grants += kernel.tick(0).await.run.len(),
                "bind" => {
                    let owner = reservation.take().unwrap();
                    let key = binding(&owner);
                    assert_eq!(kernel.bind_activation(owner, key).await.unwrap(), actor.unwrap());
                    assert_eq!(kernel.transport_key(actor.unwrap()), Some(key));
                    assert_eq!(kernel.request_exclusive(actor.unwrap()).await, Err(KernelError::InvalidTransition));
                    kernel.release_exclusive(actor.unwrap()).await;
                    assert_eq!(kernel.shards.shard_of(actor.unwrap()).await, Some(key.shard));
                }
                "abort" => kernel.abort_activation(reservation.take().unwrap()).await.unwrap(),
                "attempt-transitions" => {
                    let actor = actor.unwrap();
                    assert_eq!(kernel.suspend(actor, None).await, Err(KernelError::InvalidTransition));
                    assert_eq!(kernel.resume(actor).await, Err(KernelError::InvalidTransition));
                    assert_eq!(kernel.request_exclusive(actor).await, Err(KernelError::InvalidTransition));
                    assert_eq!(kernel.link_extension(actor, actor).await, Err(KernelError::InvalidTransition));
                    let result = TurnResult {
                        ui_patches: vec![],
                        effects: vec![],
                        command_ingress: vec![],
                        cold_pair_ingress: Default::default(),
                        lifecycle_receipt: None,
                        ui_patch_receipt: None,
                        next_wake: None,
                        status: TurnStatus::Idle,
                        usage: Usage::default(),
                    };
                    assert_eq!(kernel.complete(actor, &result, 0).await, Err(KernelError::InvalidTransition));
                    assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Activating));
                    assert_eq!(kernel.shards.shard_of(actor).await, Some(reservation.as_ref().unwrap().shard()));
                }
                "quarantined-bind" => {
                    let owner = reservation.take().unwrap();
                    kernel.quarantine_package(&request().package, 0).await;
                    let key = binding(&owner);
                    let refused = kernel.bind_activation(owner, key).await.expect_err("quarantine must survive physical admission");
                    assert_eq!(kernel.actor_status(actor.unwrap()).await, Some(&ActorStatus::Quarantined));
                    reservation = Some(refused.reservation);
                }
                "foreign-bind" => {
                    let mut foreign = Kernel::new(ShardKind::Native, 1, 0, 4).await;
                    let owner = foreign.reserve_activation(request()).await.unwrap();
                    let key = binding(&owner);
                    assert_eq!(key.actor, actor.unwrap());
                    let refusal = kernel.bind_activation(owner, key).await.unwrap_err();
                    assert_eq!(refusal.reason, KernelActivationFault::ForeignReservation);
                    foreign.abort_activation(refusal.reservation).await.unwrap();
                    assert!(kernel.transport_key(actor.unwrap()).is_none());
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(grants as u64, row["grants"].as_u64().unwrap(), "{}", row["id"]);
        assert_eq!(kernel.actor_record(actor.unwrap()).await.is_some(), row["actorRetained"].as_bool().unwrap(), "{}", row["id"]);
        eprintln!("[DEBUG] Kernel reservation trace={} grants={grants} retained={}", row["id"], row["actorRetained"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn reservation_exhaustion_collision_and_stale_binding_leave_no_partial_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap();
    let mut kernel = Kernel::new(ShardKind::Native, 1, 0, 4).await;
    kernel.next_ordinal.insert(request().package, fixture["ordinalMaximum"].as_u64().unwrap().try_into().unwrap());
    let refused = kernel.reserve_activation(request()).await.unwrap_err();
    assert_eq!(refused.reason, KernelActivationFault::Exhausted);
    assert!(kernel.actors.is_empty() && kernel.scheduler.actors.is_empty());
    kernel.next_ordinal.clear();
    let owner = kernel.reserve_activation(request()).await.unwrap();
    let mut other = request();
    other.package = PackageId("same-plugin-ordinal".into());
    let refused = kernel.reserve_activation(other).await.unwrap_err();
    assert_eq!(refused.reason, KernelActivationFault::Occupied);
    assert!(!kernel.next_ordinal.contains_key(&refused.request.package));
    let wrong = ActorShardKey { shard: ShardId(9), ..binding(&owner) };
    let refused = kernel.bind_activation(owner, wrong).await.unwrap_err();
    assert_eq!(refused.reason, KernelActivationFault::WrongBinding);
    assert!(!kernel.scheduler.actors[&refused.reservation.actor()].active);
    let owner = refused.reservation;
    let stale = KernelActivationReservation { actor: owner.actor, shard: owner.shard, authority: Arc::clone(&owner.authority) };
    let key = binding(&owner);
    kernel.abort_activation(owner).await.unwrap();
    assert!(kernel.shards.shard_of(key.actor).await.is_none());
    let refused = kernel.bind_activation(stale, key).await.unwrap_err();
    assert_eq!(refused.reason, KernelActivationFault::ForeignReservation);
    assert!(kernel.actors.is_empty() && kernel.scheduler.actors.is_empty());
    eprintln!("[DEBUG] Kernel reservation exhaustion=1 collision=1 wrong-binding=1 stale-binding=1 partial-admissions=0");
}
