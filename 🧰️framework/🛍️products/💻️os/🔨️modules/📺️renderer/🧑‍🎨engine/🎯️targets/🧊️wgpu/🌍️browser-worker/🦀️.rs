//! 🌍️ Browser renderer worker and bootstrap.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

#![cfg(target_arch = "wasm32")]

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_app_runtime::AppRuntime;
use semio_framework_os_renderer_wgpu_asset_authority::{RendererAssetFetchOwner, RendererAssetSealStep};
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners};
use semio_framework_os_renderer_wgpu_frame_job::frame_authority;
use semio_framework_os_renderer_wgpu_frame_job::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT;
use semio_framework_os_renderer_wgpu_frame_presentation::AppPresenter;
use semio_framework_os_renderer_wgpu_host_winit::os_host;
use semio_framework_os_renderer_wgpu_host_winit::os_host::{OsHost, OsHostRetirement};
use semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimeMailbox;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_input::PointerCapture;
use semio_framework_os_renderer_wgpu_shell_prefs::shell_language_axes;
use semio_framework_os_renderer_wgpu_shell_session::declare_boot_subphase;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use infinite_world::world::{WORLD_ASSET_RESPONSE_PAGE_BYTES, WorldAssetRequestKind, WorldAssetResponsePage};
#[cfg(not(target_arch = "wasm32"))]
use infinite_world::world::WORLD_ASSET_RESPONSE_BYTE_CAPACITY;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{appearance_is_dark, frame_latency};
use semio_framework_os_renderer_wgpu_host_plumbing::{browser_original_output, input_wire};
use semio_framework_os_renderer_wgpu_program_bridge::filter_plugins;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_program_bridge::parse_plugin_entries;
use ui_gpu::GpuContext;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{InputState, PointerModifiers};
use semio_framework_ui_theme::Theme;

#[path = "../🌐️browser-worker/🦀️.rs"]
pub mod browser_worker;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
