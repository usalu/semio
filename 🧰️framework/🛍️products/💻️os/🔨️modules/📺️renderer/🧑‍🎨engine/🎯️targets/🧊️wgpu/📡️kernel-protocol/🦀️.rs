//! 📡️ Kernel protocol: sequence authority, job-progress presentation, turn budget, extension index, requests and close submissions, the mounted product replay subsystem, response slots and exchange outcomes, the request queue, the parallel runtime and component page reading.
//!
//! 🧭️ Native only: the bottom of the actor-kernel group. The module files stay in the renderer tree; this crate root mounts them and re-exports their surface.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework::kernel::{Budget as TurnBudget, Effect, Event, JobPlacement, MessageEndpoint, TurnResult};
use semio_framework_actor::{ActorId, CapabilityGrant, JobOperation, JobProgressIdentity, JobProgressKind, JobReplayRecordHeader, JobReplayRequest, JobReplayRoute, JobTurn, Lane, PackageId, JOB_PROGRESS_ACTIVE_CAPACITY};
use semio_framework_job::RetainedCloneGrant;
use semio_framework_os_renderer_wgpu_contracts::ProgramFault;
use semio_framework_os_renderer_wgpu_host_base::{plugin_catalog, run_renderer_io};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use ui_contract::{SurfaceId, UiDocumentLease, UiFixedList, UI_DOCUMENT_LEASE_SLOTS};

#[path = "../🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs"]
mod kernel_core;
pub use kernel_core::*;

#[path = "../🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎟️task-ingress/🦀️.rs"]
mod original_task_ingress;
pub(crate) use original_task_ingress::{OriginalKernelTaskInput, OriginalKernelTaskProducer};

#[path = "📥️lifted/📬️kernel-replay-client/🦀️.rs"]
mod replay_vocabulary;
pub use replay_vocabulary::*;

#[path = "📥️lifted/🏊️kernel-pool/🦀️.rs"]
mod request_queue;
pub use request_queue::*;

#[path = "📥️lifted/📦️kernel-runtime/🦀️.rs"]
mod native_component;
pub use native_component::*;

#[path = "../🎠️runtime/🦀️.rs"]
pub mod parallel_runtime;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

pub use semio_framework_os_renderer_wgpu_host_plumbing::{native_caller, native_launch};
pub use semio_framework_os_renderer_wgpu_host_plumbing::native_io::{RendererSourceIoError, find_renderer_wasm_path, read_renderer_source_bytes};
use semio_framework_os_renderer_wgpu_host_base::{renderer_worker_pool, RENDERER_IO_WAKE};
