//! 🧪️ Genuine Wasmtime component coverage for emitted and returned patch ownership.

use super::*;
use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, ActorUiPatchReceipt, AppInstanceId, QuotaSchema};
use semio_framework_ui_contract::{UiNodeId, UiPatchOp};

const ACTIVATION_GENERATION: u64 = 41;
const OPEN_SEQUENCE: u64 = 7;
const SESSION: u64 = 810_041;

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(std::env::var_os("SEMIO_UI_PATCH_SCALE_WASM").expect("registered scale component path")).expect("read registered scale component")
}

fn budget() -> Budget {
    Budget { fuel: 50_000_000, deadline_ms: 10_000, max_effects: 8, max_patch_bytes: 2_097_152, max_frames: 1 }
}

async fn open_live(config: serde_json::Value) -> (WasmtimeRuntime, GuestInstance, ActorInstanceLifetime) {
    let bytes = fixture_bytes();
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let package = PackageRef { package: PackageId("semio:scale-ui-patch".to_string()), hash: PackageHash(*semio_framework_hash::hash(&bytes).as_bytes()) };
    let compiled = runtime.compile(&package, &bytes).await.expect("scale component compiles");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget()).await.expect("scale component instantiates");
    let open = Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: ACTIVATION_GENERATION, instance_id: 1, request_sequence: OPEN_SEQUENCE },
        app_id: AppInstanceId("scale-ui".to_string()),
        actor: "scale-ui".to_string(),
        config: serde_json::to_vec(&config).expect("config encodes"),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: QuotaSchema::default(),
    };
    let captured_turn = runtime.execute_turn(&mut instance, &[open], budget()).await.expect("open produces capture");
    assert!(captured_turn.ui_patches.is_empty());
    assert_eq!(captured_turn.ui_patch_receipt, None);
    let captured = captured_turn.lifecycle_receipt.expect("component captures lifecycle");
    let lifetime = match captured {
        ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } => {
            assert_eq!(request_sequence, OPEN_SEQUENCE);
            assert_eq!(lifetime.activation_generation, ACTIVATION_GENERATION);
            assert_eq!(lifetime.instance_id, 1);
            assert_ne!(lifetime.guest_lifetime, 0);
            lifetime
        }
        _ => panic!("open must return captured lifecycle receipt"),
    };
    let ack = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured });
    let ack_turn = runtime.execute_turn(&mut instance, &[ack], budget()).await.expect("exact capture acknowledgement");
    assert!(ack_turn.ui_patches.is_empty());
    assert_eq!(ack_turn.lifecycle_receipt, None);
    assert_eq!(ack_turn.ui_patch_receipt, None);
    (runtime, instance, lifetime)
}

fn assert_patch(result: &TurnResult, lifetime: ActorInstanceLifetime, root: u64, sequence: u64) {
    assert_eq!(result.ui_patches.len(), 1);
    assert_eq!(result.ui_patch_receipt, Some(ActorUiPatchReceipt { lifetime, patch_sequence: sequence }));
    let patch = result.ui_patches.iter().next().expect("one retained patch");
    assert_eq!(patch.surface.as_ref(), "1:map.main");
    assert_eq!(patch.base_revision.0, sequence - 1);
    assert_eq!(patch.revision.0, sequence);
    assert_eq!(patch.ops.len(), 1);
    assert!(matches!(patch.ops.get(0), Some(UiPatchOp::SetRoot { id }) if *id == UiNodeId(root)));
}

async fn assert_single_claim_transport(result: TurnResult, lifetime: ActorInstanceLifetime, root: u64, sequence: u64, session: u64) {
    assert_patch(&result, lifetime, root, sequence);
    let bridged = shard::to_actor_turn_result(result, session, 0, 0).await.expect("component patch enters exact shard owner");
    let lease = semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).expect("one exact token claim");
    assert!(semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&bridged.ui_patches, session).is_err());
    let mut owner = lease.take_owner().unwrap_or_else(|_| panic!("one exact patch owner"));
    let retained = owner.iter().next().expect("transport retains component patch");
    assert!(matches!(retained.ops.get(0), Some(UiPatchOp::SetRoot { id }) if *id == UiNodeId(root)));
    while !owner.close_step() {}
}

#[semio_framework_async_macros::async_test]
async fn genuine_component_returned_and_imported_patches_keep_channel_order_and_exact_authority() {
    for (config, root, session) in [(serde_json::json!({ "profile": "ui" }), 201, SESSION), (serde_json::json!({ "profile": "ui", "uiImportSink": true }), 101, SESSION + 1)] {
        let (runtime, mut instance, lifetime) = open_live(config).await;
        let turn = runtime.execute_turn(&mut instance, &[Event::Wake], budget()).await.expect("genuine component patch");
        assert_single_claim_transport(turn, lifetime, root, 1, session).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn imported_then_returned_channels_refuse_atomically_without_a_transport_token() {
    let (runtime, mut instance, _) = open_live(serde_json::json!({ "profile": "ui", "uiImportSink": true, "uiReturnWithImport": true })).await;
    for _ in 0..2 {
        let error = runtime.execute_turn(&mut instance, &[Event::Wake], budget()).await.expect_err("two channels exceed one logical patch");
        assert!(error.to_string().contains("unpaired-authority"));
    }
}

#[semio_framework_async_macros::async_test]
async fn malformed_import_is_drained_before_the_next_exact_patch_owner_is_published() {
    let (runtime, mut instance, lifetime) = open_live(serde_json::json!({ "profile": "ui", "uiImportSink": true, "uiMalformedFirst": true })).await;
    let error = runtime.execute_turn(&mut instance, &[Event::Wake], budget()).await.expect_err("non-canonical imported payload");
    assert!(error.to_string().contains("ui patch upsert node"));
    let recovered = runtime.execute_turn(&mut instance, &[Event::Wake], budget()).await.expect("next imported patch is isolated from malformed sink");
    assert_single_claim_transport(recovered, lifetime, 302, 2, SESSION + 2).await;
}
