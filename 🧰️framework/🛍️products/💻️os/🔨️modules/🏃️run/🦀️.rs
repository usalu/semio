//! 🕸️ Headless computation of an OS studio's workflow — no UI involved. A `SpaceRunner` walks
//! `workflow::Workflow` (the kernel-crate persisted graph — a node IS the app-instance, identified
//! solely by `WorkflowNode::id`) in topological order, drives each node's app through
//! `AppChannelHost` — the exact `protocol::AppCommand`/`AppFrame` binary channel a live UI speaks, so
//! a headless run never needs a UI-mock API — moves `Media` along edges, and skips any node whose
//! inputs, document, and config are all unchanged since the last (sealed) run. Every node's frame
//! script is `SetMergePolicy → LoadConfig → LoadDocumentArchive`, then `PollDocumentArchiveLoad*` →
//! `AcknowledgeDocumentArchiveLoad` (the stepped, cancellable whole-document load), then `MediaIn* →
//! (MediaOut+MediaFingerprint)* → ReadDocument → ReadConfig` (see `SpaceRunner::compute_node`);
//! documents/configs are addressed by their node's own `artifact_ref`/`config_ref` string, never by a
//! separate instance id.
//!
//! 🔒️ W5 Lane A ("non-destructive `SpaceRunner` rework"): a run is READONLY over its source. The
//! `documents`/`configs` maps `SpaceRunner::run` reads are NEVER written back — `ReadDocument`/
//! `ReadConfig`'s mutated bytes (whatever `MediaIn` importing changed) land in a `RunSink` instead
//! (`🔖️RunContext` below), keyed by node id, a deliberately different key space than the source
//! `artifact_ref`/`config_ref` maps so a caller cannot accidentally alias a run-owned path onto a
//! source artifact path the way the old destructive path did (pre-rework: `bin.rs` wrote a node's
//! mutated document/config bytes straight back over `artifacts/<artifact_ref>`/`artifacts/<config_ref>`
//! — the very same paths `SpaceRunner::run` had just read them from). "Pinned" source access for this
//! wave means simply "whatever the source currently is, read-only, at the moment `run` reads it" — a
//! real checkpoint-id-pinned snapshot is later-wave `space::DraftCatalog`/collection-artifact
//! integration work (see `SpaceBundle`'s own doc on `workflow_node_for_app`'s still-path-stem
//! `artifact_ref`/`config_ref`).
//!
//! 🗄️ Memoization is now keyed off the PRIOR SEALED run's own `semio_framework_artifact_workflow_run::RunArtifact.node_records`
//! (`prior_node_records`, read-only), not a side-channel `run/state.json` file — that file and its
//! `RunState`/`NodeRunRecord` types are deleted (there were two parallel memoization mechanisms before
//! this rework; now there is exactly one). A first run (no prior sealed `RunArtifact`) computes every
//! node fresh.

/// 🎞️ The exact binary channel a live UI speaks — re-exported so an `AppChannelHost` implementor
/// never needs a direct `protocol` dependency just to name these types.
pub use protocol::{AppCommand, AppFrame, CHANNEL_VERSION};
use semio_framework::{media_types_compatible, Media, MediaClass, MediaCompat, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, PackageDescriptor, PluginManifest, PortMultiplicity};
// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the reactor-ABI event/
// effect vocabulary `WasmtimeNodeHost::open`/`run_turn`/`exchange` now speak — the same family of
// names `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🦀️.rs`'s `kernel_runtime` module imports from this same path, for the
// same reason (driving a real turn through `semio_framework_actor::Kernel`); this crate uses
// `Effect::Respond`/`RequestOutcome` (per-command reply correlation) rather than that consumer's
// `Effect::SendMessage`/`MessageEndpoint` (UI-surface push messaging, which `run` has no use for).
use semio_framework::kernel::{AppInstanceId, Effect, Event, QuotaSchema, RequestOutcome};
use semio_framework_actor::ActorId as RuntimeActorId;
use semio_framework_actor::{ActivationEvent, ActorKind, Backpressure, Envelope, Lane, Origin, PackageId, Payload};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
// 🎛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (dedyn-fw-os-guestruntime): trait import needed for
// `.compile`/`.instantiate` method-call syntax now that `runtime` below is the concrete
// `GuestRuntimes` enum, not `dyn GuestRuntime` (a `dyn Trait` receiver resolves its methods without
// the trait in scope; a concrete enum implementing it does not).
use semio_framework_plugin_host::GuestRuntime;
// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the shared
// kernel-activation facade — see `semio-framework-os`'s own `🎠️activation/🦀️.rs` module doc for why it
// lives in the product's host crate rather than in the wgpu target's `ParallelRuntime`.
use semio_framework_os::activation::NativeKernelRuntime;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use store::{BlobStore, NoBlobStore};
#[cfg(test)]
use semio_framework_artifact_workflow_run::{SealRun, StartRun};

use semio_framework_artifact_workflow_run::{FinishRunNode, PortFingerprint, RunMutation, RunNodeRecord, RunNodeStatus, RunOutputArtifact, RunParameterValue, StartRunNode};
use workflow::{MediaContract, Workflow, WorkflowEdge, WorkflowNode, WorkflowParameterBinding};

//#region 🔖️Children

#[path = "🚧️run-errors/🦀️.rs"]
mod run_errors;
pub use run_errors::*;

#[path = "📡️channel-ports/🦀️.rs"]
mod channel_ports;
pub use channel_ports::*;

#[path = "🎞️media/🦀️.rs"]
mod media;
pub use media::*;

#[path = "🧾️run-context/🦀️.rs"]
mod run_context;
pub use run_context::*;

#[path = "🗃️space-bundle/🦀️.rs"]
mod space_bundle;
pub use space_bundle::*;

#[path = "🧭️topology-and-edges/🦀️.rs"]
mod topology_and_edges;
use topology_and_edges::*;

#[path = "🏃️space-runner/🦀️.rs"]
mod space_runner;
pub use space_runner::*;

#[path = "🧠️wasmtime-node-host/🦀️.rs"]
mod wasmtime_node_host;
pub use wasmtime_node_host::*;

//#endregion 🔖️Children

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
