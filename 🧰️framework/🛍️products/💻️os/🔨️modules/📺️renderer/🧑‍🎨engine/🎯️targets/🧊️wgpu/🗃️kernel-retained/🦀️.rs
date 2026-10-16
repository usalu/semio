//! 🗃️ Kernel retained state: retained document exchange and surfaces, pending rejections, mounted replay routes and command document retirement.
//!
//! 🧭️ Native only. The module file stays in the renderer tree; this crate root mounts it and re-exports its surface.

use semio_framework::kernel::{JobPlacement, UiPatch as KernelUiPatch};
use semio_framework_actor::{ActorId, JobProgressReceipt, JobPublication, JobReplayLog, JobReplayPublicationPolicy, JobReplayRequest, JobReplayRoute, JobTurn, JOB_PROGRESS_ACTIVE_CAPACITY};
use semio_framework_os_renderer_wgpu_kernel_protocol::{next_seq, reserve_seq, ExchangeSurfaceDocument, JobProgressPresentationToken};
use std::mem::ManuallyDrop;
use std::sync::{Mutex, OnceLock};
use ui_contract::{SurfaceId, UiDocumentBuilder, UiDocumentLease, UiFixedList, UiRevision, UiSnapshotState, UI_DOCUMENT_LEASE_SLOTS};

#[path = "../🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs"]
mod kernel_retained_state;
pub use kernel_retained_state::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
