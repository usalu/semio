//! 👷️ Frame build job, handle and completion wake.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

use semio_framework_os_renderer_wgpu_app_runtime::{AppFramePreparation, AppFramePresentation, AppHandle, AppMailbox, FramePreparationStep, frame_child_step, frame_inline_gate, frame_inline_progress, frame_receive_close, frame_received_child, frame_worker_child_step};
use semio_framework_os_renderer_wgpu_frame_actions::{FrameBuildInputs, FrameDirectives};
use semio_framework_os_renderer_wgpu_frame_transaction::{AppFrameTransactionStep, FrameTransaction};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{log_debug_diagnostic, log_debug_once_per_transition, renderer_worker_pool};
use std::sync::Arc;

#[path = "../🧵️frame-job/🦀️.rs"]
pub mod frame_job;
#[path = "../../../🪙️authority/🖼️frame/🦀️.rs"]
pub mod frame_authority;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
