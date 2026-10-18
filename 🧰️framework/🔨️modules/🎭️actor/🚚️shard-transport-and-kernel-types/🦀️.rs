//! 🚚️ Shard transport and kernel types: the shard transport trait with its thread transport, the activation event, the kernel error, the actor meta and the kernel struct. See [`super`].

use super::*;

//#region 🚚️ShardTransport
/// 🚚️ Injected duplex byte transport to one shard. `ThreadTransport` is the only impl living in
/// this crate (native, `std::sync::mpsc`); `WorkerTransport` (postMessage) and `ProcessTransport`
/// (stdio) are host-supplied — all three carry the same `Envelope`/`TurnResult` pack encoding.
pub trait ShardTransport: Send {
    async fn send(&self, bytes: &[u8]);
    async fn recv(&self) -> Option<Vec<u8>>;
    async fn heartbeat(&self) -> u64;
    async fn kill(&self);
}

//#region 🔖️ThreadTransport
#[cfg(not(target_arch = "wasm32"))]
pub(super) mod thread_transport {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};

    use super::ShardTransport;

    /// 🚚️ Native duplex transport over `std::sync::mpsc` — one end held by the kernel, the mirror
    /// end by the shard loop. Gated to non-wasm32 only; the pure crate core never spawns threads
    /// itself (the shard loop that reads/writes the mirror end lives in `plugin/host`, packet B1).
    pub struct ThreadTransport {
        outbound: Sender<Vec<u8>>,
        inbound: Mutex<Receiver<Vec<u8>>>,
        heartbeat_ms: Arc<AtomicU64>,
        killed: Arc<AtomicBool>,
    }

    impl ThreadTransport {
        /// 🔗️ Builds both ends of one duplex link, sharing the heartbeat/kill flags.
        pub async fn new_pair() -> (ThreadTransport, ThreadTransport) {
            let (tx_a, rx_a) = std::sync::mpsc::channel();
            let (tx_b, rx_b) = std::sync::mpsc::channel();
            let heartbeat_ms = Arc::new(AtomicU64::new(0));
            let killed = Arc::new(AtomicBool::new(false));
            let side_a = ThreadTransport { outbound: tx_a, inbound: Mutex::new(rx_b), heartbeat_ms: heartbeat_ms.clone(), killed: killed.clone() };
            let side_b = ThreadTransport { outbound: tx_b, inbound: Mutex::new(rx_a), heartbeat_ms, killed };
            (side_a, side_b)
        }

        /// 💓️ Records a heartbeat timestamp — called by whichever side is standing in for the
        /// shard loop's `Atomics.store` in the real web transport.
        pub async fn beat(&self, now_ms: u64) {
            self.heartbeat_ms.store(now_ms, Ordering::SeqCst);
        }

        /// ⏳️ Blocking receive with a timeout — `ShardExecutor` (terra-shard-grants,
        /// `🖥️host/🧵️shard/🧵️executor/🦀️.rs`) parks here between pumps instead of a caller-driven
        /// busy-poll loop. `mpsc::Receiver::recv_timeout` blocks only the CALLING thread; it spawns
        /// no thread of its own, so the purity grep this crate's core is verified against (no
        /// `std::thread` in `🦀️.rs`) still holds. Not part of the [`ShardTransport`]
        /// trait — `WorkerTransport`/`ProcessTransport` have their own, different ways of parking
        /// (a JS event loop, a blocking `read`), so this stays a `ThreadTransport`-only inherent
        /// method, the same shape `Self::beat` already uses for its own extra, non-trait surface.
        pub async fn recv_deadline(&self, timeout: std::time::Duration) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.recv_timeout(timeout).ok())
        }

        /// 🤝️ Takes one currently buffered frame without parking the caller.
        pub fn try_recv_now(&self) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.try_recv().ok())
        }

        /// 🛡️ Attempts one owned send without suspension and returns the exact frame when the
        /// transport is terminal or disconnected.
        pub fn send_now(&self, bytes: Vec<u8>) -> Result<(), Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return Err(bytes);
            }
            self.outbound.send(bytes).map_err(|error| error.0)
        }
    }

    impl ShardTransport for ThreadTransport {
        async fn send(&self, bytes: &[u8]) {
            let _ = self.send_now(bytes.to_vec());
        }

        async fn recv(&self) -> Option<Vec<u8>> {
            self.try_recv_now()
        }

        async fn heartbeat(&self) -> u64 {
            self.heartbeat_ms.load(Ordering::SeqCst)
        }

        async fn kill(&self) {
            self.killed.store(true, Ordering::SeqCst);
        }
    }
}
//#endregion 🔖️ThreadTransport
//#endregion 🚚️ShardTransport

//#region 🏛️Kernel
/// 🏛️ What activated an actor.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActivationEvent {
    Manual,
    WindowOpen { window: WindowId },
    Restart,
}

/// 🚨️ Errors the [`Kernel`] façade's fallible operations return.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelError {
    UnknownActor,
    NoExclusiveShard,
    InvalidTransition,
    TurnNotIssued,
    InvalidRetainedReceipt,
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownActor => formatter.write_str("unknown actor"),
            Self::NoExclusiveShard => formatter.write_str("no exclusive shard available"),
            Self::InvalidTransition => formatter.write_str("invalid status transition"),
            Self::TurnNotIssued => formatter.write_str("actor turn was not issued"),
            Self::InvalidRetainedReceipt => formatter.write_str("original actor retained receipt does not match issued authority"),
        }
    }
}

impl std::error::Error for KernelError {}

/// 🏛️ Per-actor bookkeeping the [`Scheduler`] doesn't need to see: kind, capabilities, status,
/// failure ladder, metrics, and which window (if any) its scene patches target.
#[derive(Clone, Debug)]
pub(super) struct ActorMeta {
    pub(super) reservation: Option<Arc<activation::ReservationAuthority>>,
    pub(super) transport_key: Option<activation::ActorShardKey>,
    pub(super) kind: ActorKind,
    pub(super) package: PackageId,
    pub(super) capabilities: Vec<CapabilityGrant>,
    pub(super) budget: Budget,
    pub(super) status: ActorStatus,
    pub(super) failure: FailureState,
    pub(super) metrics: ActorMetrics,
    pub(super) window: Option<WindowId>,
}

/// 🏛️ The one-implementation, three-host runtime façade: `submit`/`tick`/`complete`/`activate`/
/// `suspend`/`resume`/`request_exclusive`/`commit_frame`/`metrics`. Composes [`Scheduler`],
/// [`ShardTable`], per-window [`SceneStore`]s and the failure ladder over one actor registry.
#[derive(Debug)]
pub struct Kernel {
    pub(super) scheduler: Scheduler,
    pub(super) shards: ShardTable,
    pub(super) scenes: HashMap<WindowId, SceneStore>,
    pub(super) actors: HashMap<ActorId, ActorMeta>,
    pub(super) next_ordinal: HashMap<PackageId, u32>,
    /// 🔗️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): explicit parent→children
    /// edge table for extension cascade — see `//#region 🔗️ExtensionActivation` below. Kept explicit
    /// rather than re-derived from `ActorKind::Extension.plugin` on every cascade so that (a) cascade is
    /// O(children) instead of an O(actors) scan, and (b) it survives multiple *instances* of one plugin
    /// (several `PluginApp` actors sharing a `PackageId`, each with its own extension subtree) without
    /// the `plugin: PackageId` field alone being able to disambiguate which instance a child belongs to.
    pub(super) links: HashMap<ActorId, Vec<ActorId>>,
}
//#endregion 🏛️Kernel
