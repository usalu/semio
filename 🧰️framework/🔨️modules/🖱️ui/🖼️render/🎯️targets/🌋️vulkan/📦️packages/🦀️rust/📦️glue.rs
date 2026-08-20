//! @emoji 🌋️ Hand-written Vulkan backend for Linux.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! **Every module below is `target_os = "linux"`-gated, not just this banner.** `Cargo.toml` puts
//! `ash`/`ash-window`/`raw-window-handle` behind `[target.'cfg(target_os = "linux")'.dependencies]`, so
//! on any other host those crates are not even in the dependency graph — a bare `compile_error!` with
//! ungated `mod` declarations underneath it would still try to resolve `use ash::vk;` and fail with
//! cascading "can't find crate" errors on top of the intentional one. Gating the `mod` statements
//! themselves means a native macOS `cargo check` sees exactly one error (this banner) and nothing else,
//! while `cargo check --target x86_64-unknown-linux-gnu` compiles the real crate.
//!
//! See `🦀️backend.rs`'s header for which milestone this crate reaches and
//! `📓️terra-backend-vulkan-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

#[cfg(not(target_os = "linux"))]
compile_error!("semio-framework-ui-backend-vulkan builds only on Linux.");

#[cfg(target_os = "linux")]
#[path = "🦀️memory.rs"]
mod memory;
#[cfg(target_os = "linux")]
#[path = "🦀️vk_error.rs"]
mod vk_error;
#[cfg(target_os = "linux")]
#[path = "🦀️swapchain_support.rs"]
mod swapchain_support;
#[cfg(target_os = "linux")]
#[path = "🦀️descriptor_layout.rs"]
mod descriptor_layout;
#[cfg(target_os = "linux")]
#[path = "🦀️resources.rs"]
mod resources;
#[cfg(target_os = "linux")]
#[path = "🦀️backend.rs"]
mod backend;

#[cfg(target_os = "linux")]
pub use backend::VulkanBackend;
#[cfg(target_os = "linux")]
pub use vk_error::VulkanGraphicsError;
