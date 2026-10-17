//! 🍇️ Companion test bin of the wgpu renderer: frame core and renderer laws (1).
//!
//! 🧭️ The tests of this bin were mounted by the renderer root and its hubs; they are mounted from their place in the renderer tree here, and the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_renderer_wgpu_asset_decode::{GLB_OUTLINE_SCALE, GlbMatrix, GlbOutlineAccumulator};
use semio_framework_os_renderer_wgpu_canvas2d_host::begin;
use semio_framework_os_renderer_wgpu_frame_actions::{FrameActionOwners, FrameDeferredCursor, FrameDeferredWork, FrameInputActionStep, RuntimeDispatchCursor, RuntimeDispatchSource, transfer_frame_input_action};
use semio_framework_os_renderer_wgpu_frame_presentation::{APP_PRESENT_STALL_STEPS, AppPresentInputWait, AppPresentPhase, AppPresentProgress, AppPresentStallWatch, note_present_stall_signature};
use semio_framework_os_renderer_wgpu_frame_transaction::FrameTransactionStage;
use semio_framework_os_renderer_wgpu_host_base::frame_latency::*;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_probe::native_socket_sqlite;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_probe::{NativeSocketProbeDiff, NativeSocketProbeMutation, NativeSocketProbeSnapshot};
use semio_framework_os_renderer_wgpu_shell_sync::Phase;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_value::{DslValue, FromValue, ToValue};
use serde::Deserialize;
use serde_json::Value;
use std::sync::atomic::Ordering;
use ui_wgpu::wgpu::{ActionDescriptor, InputState};
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;

#[path = "../../../🧪️tests/🧾️frame-action-ledger/🦀️.rs"]
mod frame_action_ledger_tests;
#[path = "../../../🧪️tests/🖱️wheel-application-point/🦀️.rs"]
mod wheel_application_point_tests;
#[path = "../../../🧪️tests/⏱️frame-latency-diagnostics/🦀️.rs"]
mod frame_latency_diagnostics_tests;
#[path = "../../../🧪️tests/🧭️boot-axis-parity/🦀️.rs"]
mod boot_axis_parity_tests;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🧊️renderer/🪶️sqlite/🧪️tests/🦀️.rs"]
mod native_socket_sqlite_tests;
include!("../../../🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs");
include!("../../../🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs");
