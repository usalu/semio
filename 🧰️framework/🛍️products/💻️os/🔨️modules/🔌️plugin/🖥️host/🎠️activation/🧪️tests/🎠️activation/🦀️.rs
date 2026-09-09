use super::*;
use crate::component::{MockGuestRuntime, PackageHash, PackageRef};
use crate::shard::executor::OutcomeSink;
use semio_framework_actor::{ActivationEvent, ActorKind, Lane, PackageId, ShardKind};
use semio_framework_async::{ProcessKind, WorkerPool, WorkerPoolConfig};

#[semio_framework_async_macros::async_test]
async fn neutral_activation_failures_retire_the_exact_kernel_and_guest_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::clone(&mock)));
        let pool = Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2)));
        let mut kernel = Kernel::new(ShardKind::Native, 1, 0, 8).await;
        let package = PackageId("activation-ownership".into());
        let request = semio_framework_actor::activation::KernelActivationRequest {
            package: package.clone(),
            plugin_ordinal: 0,
            kind: ActorKind::PluginApp { plugin: package.clone(), app_id: "test".into(), instance_id: 1 },
            lane: Lane::Interactive,
            window: None,
            event: ActivationEvent::Manual,
        };
        let reservation = kernel.reserve_activation(request).await.unwrap();
        let actor = reservation.actor();
        let compiled = mock.compile(&PackageRef { package: package.clone(), hash: PackageHash([0; 32]) }, &[]).await.unwrap();
        let stage = row["stage"].as_str().unwrap();
        let shards = if stage == "shard" { Vec::new() } else { vec![ShardExecutor::new(Arc::clone(&pool), Arc::clone(&runtime), Vec::new(), OutcomeSink::new()).await] };
        if stage == "record" {
            kernel.deactivate(actor).await.unwrap();
        }
        if stage == "quarantine" {
            let peer = kernel.activate(package.clone(), 0, ActorKind::PluginApp { plugin: package.clone(), app_id: "peer".into(), instance_id: 2 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let fault = semio_framework_actor::TurnResult {
                ui_patches: vec![],
                effects: vec![],
                command_ingress: vec![],
                cold_pair_ingress: Default::default(),
                lifecycle_receipt: None,
                ui_patch_receipt: None,
                next_wake: None,
                status: semio_framework_actor::TurnStatus::Faulted { detail: b"trap".to_vec() },
                usage: Default::default(),
            };
            for _ in 0..semio_framework_actor::FAILURE_QUARANTINE_RESTART_THRESHOLD {
                kernel.complete(peer, &fault, 0).await.unwrap();
            }
            assert_eq!(kernel.actor_status(actor).await, Some(&semio_framework_actor::ActorStatus::Quarantined));
            kernel.deactivate(peer).await.unwrap();
        }
        if stage == "instantiate" {
            mock.fail_instantiate.store(true, std::sync::atomic::Ordering::Release);
        }
        if stage == "register" {
            pool.shutdown().unwrap();
        }
        let budget = Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let result = install_actor(&mut kernel, &runtime, &shards, reservation, &compiled, &[], &budget).await;
        assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(kernel.transport_key(actor).is_some(), result.is_ok());
        assert_eq!(kernel.actor_record(actor).await.is_some(), row["actorRetained"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(mock.instantiate_admissions.load(std::sync::atomic::Ordering::Acquire) as u64, row["instantiations"].as_u64().unwrap(), "{}", row["id"]);
        assert_eq!(mock.drop_admissions.load(std::sync::atomic::Ordering::Acquire) as u64, row["drops"].as_u64().unwrap(), "{}", row["id"]);
        eprintln!("[DEBUG] activation ownership case={} admitted={} kernel-retained={} drops={}", row["id"], result.is_ok(), kernel.actor_record(actor).await.is_some(), row["drops"]);
    }
}
