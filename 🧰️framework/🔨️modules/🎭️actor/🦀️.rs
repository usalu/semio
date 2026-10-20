//! 🎭️ Pooled actor/plugin runtime kernel — domain-neutral, pure: no I/O, no clock (callers pass
//! `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file. Transports are
//! injected via the [`ShardTransport`] trait; the one exception, [`ThreadTransport`], lives behind
//! `#[cfg(not(target_arch = "wasm32"))]` because it is built on `std::sync::mpsc`.
//!
//! 🪡 **Opaque seam**: `Effect`, `Event` and `UiPatch` are owned by a parallel packet in
//! `🎠️kernel` and are NOT defined here. This crate carries them as pack-encoded `Vec<u8>` inside
//! [`TurnResult`] (`ui_patches`, `effects`) and [`Payload::Event`] — the kernel crate encodes/decodes
//! the concrete typed collections; this crate only ever moves already-encoded bytes. The same seam
//! applies to `CapabilityGrant`: `kernel::CapabilityGrant` (`🛂️manifest/🦀️.rs`) is an
//! application-layer type this framework-tier crate must not depend on (would invert the
//! framework → os-product layering), so [`CapabilityGrant`] here is a minimal, local, pack-codeable
//! stand-in reconciled by the `B1-host-native` packet at integration time.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md` §1.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::Arc;

use semio_framework_value_derive::{FromValue, ToValue};

pub use semio_framework_job as job;
pub use semio_framework_actor_retained_turn as retained_turn;
pub use semio_framework_actor_retained_context as retained_context;
pub use semio_framework_actor_pack as pack;
pub use semio_framework_actor_vocabulary::*;
pub use semio_framework_actor_job_publication::*;
pub use retained_turn::{OriginalActorParkRefusal, OriginalActorParkedTurn, OriginalActorParkedOutcome, OriginalActorParkedReturn, IssuedRetainedBatch, IssuedRetainedRecipient, OriginalActorTurnLoan, OriginalActorTurnWork, RetainedTurnInput, RetainedTurnReceipt, OriginalActorInput, ActorGuestPhase, OriginalActorGuestReturn, OriginalActorJobProvider, OriginalJobStep, OriginalJobOutputState, OriginalJobOutcome, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputAckReply, OriginalJobSource, OriginalJobSourceCursor, OriginalJobTurnLoan, OriginalJobBudget, OriginalJobStartReply, OriginalJobStepReply, OriginalJobCancelReply, OriginalJobCallFailure};
pub use retained_turn::{OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobSourceWireCursor, OriginalJobSourceWireHeader};

#[path = "🖼️wire-turn/📬️typed-operation/🦀️.rs"]
pub mod typed_operation_page;

#[path = "🖼️wire-turn/⚠️diagnostic/🦀️.rs"]
pub mod typed_operation_diagnostics;

#[path = "🎠️activation-reservation/🦀️.rs"]
pub mod activation;

#[path = "🗂️actor-record-and-shard-table/🦀️.rs"]
mod actor_record_and_shard_table;
pub use actor_record_and_shard_table::{ActorRecord, ShardTable};

#[path = "⏱️scheduler-and-scene/🦀️.rs"]
mod scheduler_and_scene;
pub use scheduler_and_scene::{Decision, SceneSnapshot, SceneStore, Scheduler, TurnGrant};

#[path = "📈️runtime-metrics/🦀️.rs"]
mod runtime_metrics;
pub use runtime_metrics::{ActorMetrics, ActorMetricsSample, KernelMetrics, RUNTIME_METRICS_PUBLISH_INTERVAL_MS, RuntimeMetricsSnapshot, ShardMetrics, ShardMetricsSample, runtime_metrics_due};

#[path = "🚚️shard-transport-and-kernel-types/🦀️.rs"]
mod shard_transport_and_kernel_types;
use shard_transport_and_kernel_types::ActorMeta;
#[cfg(not(target_arch="wasm32"))]
use shard_transport_and_kernel_types::thread_transport;
pub use shard_transport_and_kernel_types::{ActivationEvent, Kernel, KernelError, ShardTransport};

#[path = "🏛️kernel-core/🦀️.rs"]
mod kernel_core;

#[path = "🌿️kernel-extension-activation/🦀️.rs"]
mod kernel_extension_activation;

//#endregion 🗂️ActorRecord

//#endregion 🧩️ShardTable

//#region 🚚️ShardTransport

//#region 🔖️ThreadTransport
#[cfg(not(target_arch = "wasm32"))]
pub use thread_transport::ThreadTransport;
//#endregion 🔖️ThreadTransport
//#endregion 🚚️ShardTransport

#[cfg(test)]
#[global_allocator]
static ACTOR_TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "🧪️tests/🎟️retained-turn/🦀️.rs"]
mod retained_turn_tests;
