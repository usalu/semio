//! 🖥️ Plugin host facade: the sandboxed Wasmtime component plugin host assembled from its producer crates, with the session, metrics, activation and package-asset glue of the host itself.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🦀️.rs"]
mod component;
pub use component::{ArtifactSession, RuntimeMetricsPublisher, SessionLanePack, decode_conflicts, decode_dispatch_report, decode_merge_report};
#[cfg(not(target_arch = "wasm32"))]
pub use component::{activation, package_assets};
pub use semio_framework_actor::{PackageHash, PackageId};
pub use semio_framework_plugin_guest_contract::original_actor_context;
pub use semio_framework_plugin_host_engine::{IsolatedCompileTarget, SharedEngineConfig, compile_component_isolated, compiled_cache_scratch_path, default_compiled_cache_root};
pub use semio_framework_plugin_host_guest_runtime::{CompiledHandle, GuestActorOpening, GuestInstance, GuestRuntime, GuestRuntimes, GUEST_CODEC_BUDGET, self_issued_budget, OwnedComponentDocumentCodec, PackageRef, WasmtimeRuntime, retryable_lifecycle_turn};
pub use semio_framework_plugin_host_post_turn_relay::{GuestRelayWakeAuthority, PluginInstanceHandle};
pub use semio_framework_plugin_host_routers::{AppRouter, ArtifactInferenceRouter, ArtifactMutationRouter, HostMutationInputs, HostMutationRosterEntry, HostTransactionCoordinator, InstanceDirectory, IoRouter, OpeningResolver, PluginGraph, TransactionError, TransactionMember, TransactionOutcome};
pub use semio_framework_plugin_host_wake::{GuestRelayDriverPolicy, GuestRelayWakeDriver};

pub mod shard {
    pub use semio_framework_plugin_host_shard::{OriginalShardIdentity, OriginalShardIdentityIssuer, ShardFrame, ShardIdentityPolicy, ShardOutcome, grant, native_identity_issuer, to_actor_turn_result};
    pub mod executor {
        pub use semio_framework_plugin_host_shard_executor::{OutcomeSink, ShardExecutor};
    }
}

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the one world's contract-parity
/// test (effect ↔ host-async import parity, plus the collapsed shape itself) — mounted here rather than inside `🦀️.rs` (other packets are live
/// in that file). The test uses a narrow owned WIT source inspector and adds no external parser to
/// the test graph.
#[cfg(test)]
#[path = "../../🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_parity;
