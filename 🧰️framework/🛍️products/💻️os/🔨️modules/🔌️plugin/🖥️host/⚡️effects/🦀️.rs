//! ⚡️ Async effect execution (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet terra-effects-async,
//! `📓️design-runtime.md` §2). [`AsyncEffectExecutor::execute`] is the seam a live turn loop calls
//! AFTER a turn completes: it classifies each `Effect`, derives an `OperationContext` for it, and
//! `spawn_scoped`s it into the emitting actor's own scope — cheap and synchronous itself (pure
//! classification + a `HostAsyncRuntime::spawn_scoped` call per effect), never awaiting the real
//! work inline. The real work runs later, off this call stack, on `semio-framework-os-services`'
//! pools (`HttpPool`/`StorageScheduler`/`TimerWheel`/`ComputePool`/`EventRouter`).
//!
//! 🔁️ **Completions re-enter as envelopes, never a side channel.** Every dispatched operation ends
//! by building a `semio_framework::kernel::Event` (`Event::Completed`/`Event::Timer`/`Event::Message`),
//! JSON-encoding it (the SAME encoding `🧵️shard/🧵️executor/🦀️.rs` and `🧵️shard/🚚️process-transport/
//! 🦀️.rs` already use for `Payload::Event { bytes }`), and handing it to
//! [`EnvelopeCompletionSink`] — the ONE place in this module that constructs an `Envelope` and the
//! ONE place that decides whether it is delivered now, buffered (suspended actor), or dropped
//! (stale generation). `EnvelopeCompletionSink` implements `semio_framework_os_services::
//! CompletionSink`, the only re-entry path those services are allowed to use.
//!
//! 🚰️ **Completions ride the same mailbox machinery as ordinary events.** Rather than a bespoke
//! buffer, every actor's pending completions live in a private `EventRouter` topic
//! (`ChannelPolicy::LosslessBounded`) — a floods-of-completions burst is bounded/backpressured by
//! the exact same vocabulary `EventRouter` already enforces for everything else (see
//! `#region 🚰️CompletionMailbox`'s own doc).
//!
//! 🌉️ **`EnvelopeInjector` is the seam this module does NOT close.** No live `Kernel`/shard loop is
//! driven by any code in this whole repository outside tests today (`grep -rn "Kernel::new("`
//! finds zero non-test call sites) — so there is nowhere real yet for a finished `Envelope` to go.
//! [`EnvelopeInjector`] is the trait the eventual kernel-loop/shard owner implements against its own
//! `ShardTransport`; this packet ships the executor, the sink, and a thoroughly unit-tested contract
//! against a recording double — see the packet report's `## honest gaps`.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as store;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use semio_framework::kernel::{Effect, Event, MessageEndpoint, RequestId, RequestOutcome};
use semio_framework::{DslValue, MediaType};
use semio_framework_actor::{ActorId as RuntimeActorId, Envelope, Lane as ActorLane, Origin, PackageId, Payload};
use semio_framework_async::{CancelToken, CapabilityTokenId, ChannelPolicy, HostAsyncRuntime, HostFuture, OperationContext, ScopeDrainReport, ScopeHandle, ScopeOwner, TraceId};
#[cfg(test)]
use semio_framework_job::CommitCandidate;
use semio_framework_job::{JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedPayloadBuilder, InteractiveJob, InteractiveJobCloseStep, JobPayloadStream, RetainedJobPayload, StepContext};
#[cfg(test)] use semio_framework_job::{JobFault, RetainedJobPayloadWriter, StepOutcome};
use semio_framework_os_services::{
    CompletionSink, ComputeError, ComputePool, EventRouter, HttpPool, HttpPoolError, HttpRequest as ServiceHttpRequest, HttpResponse as ServiceHttpResponse, PublishOutcome, StorageError, StorageScheduler, TimerError, TimerWheel, Topic,
};
use semio_framework_value_derive::ToValue;

#[path = "🆔️actor-registries-and-envelope-injector/🦀️.rs"]
mod actor_registries_and_envelope_injector;
pub use actor_registries_and_envelope_injector::{ActorScopeRegistry, COMPLETION_MAILBOX_CAP, COMPLETION_MAILBOX_MAX_BYTES, CapabilityRevocationRegistry, EnvelopeInjector, RecordingEnvelopeInjector, TraceIdAllocator};
use actor_registries_and_envelope_injector::{addressed_actor_id, clamp_deadline_ms, completion_topic, decode_mailbox_entry, encode_mailbox_entry};

#[path = "🧾️completion-sink-and-fault-encoding/🦀️.rs"]
mod completion_sink_and_fault_encoding;
pub use completion_sink_and_fault_encoding::EnvelopeCompletionSink;
use completion_sink_and_fault_encoding::{emit_completed_err, emit_completed_ok, encode_event};

#[path = "🌉️router-effect-handler-and-jobs/🦀️.rs"]
mod router_effect_handler_and_jobs;
pub use router_effect_handler_and_jobs::{RouterEffect, RouterEffectHandler, RouterEffectJobOutcome, UnwiredRouterEffectHandler, run_router_effect_job};
#[cfg(test)]
use router_effect_handler_and_jobs::{CompleteRouterEffectJob, DynRouterEffectJob, FaultRouterEffectJob};

#[path = "💾️storage-capabilities-backbone-and-metrics/🦀️.rs"]
mod storage_capabilities_backbone_and_metrics;
pub use storage_capabilities_backbone_and_metrics::{AllowAllCapabilities, BackboneError, BackboneRegistry, BackboneTransport, CapabilityChecker, DrainOwner, EffectMetricsRecorder, NullMetricsRecorder, RecordingMetrics, StorageBackend, UnwiredStorageBackend};

#[path = "🎛️async-effect-executor/🦀️.rs"]
mod async_effect_executor;
pub use async_effect_executor::{AsyncEffectExecutor, AsyncServices, EffectDispatchContext, EffectDispatchReport};

//#region 🧬️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧬️Tests
