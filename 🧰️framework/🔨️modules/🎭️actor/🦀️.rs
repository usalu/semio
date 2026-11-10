//! 🎭️ Pooled actor/plugin runtime kernel — domain-neutral, pure: no I/O, no clock (callers pass [Design notes](README.md#source-notes-1).

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::Arc;

use semio_framework_value_derive::{FromValue, ToValue};

pub use semio_framework_job as job;
pub use semio_framework_actor_retained_turn as retained_turn;
pub use semio_framework_actor_retained_context as retained_context;
pub use semio_framework_actor_pack as pack;
pub use semio_framework_actor_vocabulary::*;
pub use semio_framework_actor_job_publication::*;
pub use retained_turn::{OriginalActorParkAdmission,OriginalActorParkAdmissionFailure, OriginalActorParkedTurn, OriginalActorParkedOutcome, OriginalActorParkedReturn, IssuedRetainedBatch, IssuedRetainedRecipient, OriginalActorTurnLoan, OriginalActorTurnWork, RetainedTurnInput, RetainedTurnReceipt, OriginalActorInput, ActorGuestPhase, OriginalActorGuestReturn, OriginalActorJobProvider, OriginalJobStep, OriginalJobOutputState, OriginalJobOutcome, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputAckReply, OriginalJobSource, OriginalJobSourceCursor, OriginalJobTurnLoan, OriginalJobBudget, OriginalJobStartReply, OriginalJobStepReply, OriginalJobCancelReply, OriginalJobCallFailure};
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
#[path = "🧪️tests/🎟️retained-turn/🦀️.rs"]
mod retained_turn_tests;
