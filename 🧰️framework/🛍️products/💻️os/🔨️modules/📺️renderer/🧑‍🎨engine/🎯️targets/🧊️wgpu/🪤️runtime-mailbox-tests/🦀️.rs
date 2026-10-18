//! 🪤️ Companion tests of runtime-mailbox: async boundary and enqueue.
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use infinite_world::world::DecodedReferenceImage;
use infinite_world::world::{
    WORLD_ASSET_REQUEST_CAPACITY, WORLD_ASSET_RESPONSE_PAGE_BYTES, World3dState, WorldAssetIoAuthority, WorldAssetRequestKind, WorldAssetResponsePage, begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh,
    reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response, step_world3d_dynamic_retirement, take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty,
};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_app_runtime::{AppMailbox, AppRuntime, FRAME_ENGINE_PACKET_CAPACITY, FRAME_ENGINE_PACKET_PAGE_SLOTS, FRAME_ENGINE_PACKET_PAGES, FrameEnginePackets};
use semio_framework_os_renderer_wgpu_asset_authority::{JpegStructureCursor, PngStructureCursor, ProtobufStructureCursor, RENDERER_ASSET_PARSE_BLOCK_BYTES, RENDERER_ASSET_PIXEL_BYTES, RendererAssetFetchOwner, RendererAssetFormatCursor};
use semio_framework_os_renderer_wgpu_asset_decode::{GlbMaterializePhase, RendererAssetDecodeJob, RendererAssetDecodeResult, RendererAssetDecodeSession, RendererAssetProbe, RendererAssetProbeStep};
use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas;
use semio_framework_os_renderer_wgpu_frame_actions::{
    AppInteractionState, FrameActionOwners, FrameDeferredCursor, FrameDeferredWork, FrameInputActionStep, RuntimeDispatchCursor, RuntimeDispatchSource, WORLD3D_DEADLINE_CAPACITY, transfer_frame_input_action,
};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_frame_actions::{FrameExecutionOwnerCell, FrameExecutionRegistry, FrameMaintenanceAuthority, frame_maintenance_terminal_fault};
use semio_framework_os_renderer_wgpu_frame_job::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT;
use semio_framework_os_renderer_wgpu_frame_job::{frame_authority, frame_job};
use semio_framework_os_renderer_wgpu_frame_transaction::{AppFrameTransactionStep, FrameTransaction};
use semio_framework_os_renderer_wgpu_icon_host::icon_export;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_renderer_wgpu_runtime_mailbox::StagedReferenceImageAuthority;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_runtime_mailbox::{FrameDeferredExecutionOwner, FrameMaintenanceOwner, NativeReferenceDecodeCloseStep, NativeReferenceDecodeJob, NativeReferenceDecodeOutput, push_renderer_asset_page};
use semio_framework_os_renderer_wgpu_runtime_mailbox::{
    PresenterInteractionStep, RUNTIME_COMPLETION_CAPACITY, RuntimeApply, RuntimeCompletion, RuntimeCompletionQueue, RuntimeDispatchReturn, RuntimeHostWaker, RuntimeMailbox, RuntimePresentationAuthority, RuntimePresentationWitness, RuntimeRasterOperationAuthority,
    returned_completion,
};
use semio_framework_os_renderer_wgpu_shell_input::PointerCapture;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_ui_scene_mesh::Mesh3dItem;
use serde_json::Value;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, IconAtlas, InputState, Mesh3dField, PointerModifiers, SceneRasterDescriptor, SceneRasterPool, SceneRasterProfile, Theme};

#[path = "../../../🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs"]
mod async_boundary_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../../../../../🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs"]
mod runtime_publication_tests;
