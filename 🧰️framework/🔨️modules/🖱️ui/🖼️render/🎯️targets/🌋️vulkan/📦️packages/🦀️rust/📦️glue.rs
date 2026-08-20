//! @emoji 🌋️ Hand-written Vulkan backend for Linux.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! **Every module below is `target_os = "linux"`-gated.** `Cargo.toml` puts `ash`/`ash-window`/
//! `raw-window-handle` behind `[target.'cfg(target_os = "linux")'.dependencies]`, so on any other host
//! those crates are not even in the dependency graph — ungated `mod` declarations would still try to
//! resolve `use ash::vk;` and fail with cascading "can't find crate" errors.
//!
//! **On a non-Linux host this crate compiles to an empty, zero-item lib — deliberately, not an
//! oversight.** It used to gate on a hard `compile_error!` instead, but that made `cargo check
//! --workspace` (this refactor's exit gate) fail on every non-Linux host merely because this crate is a
//! workspace member, independent of whether anything actually depends on it. Wrong-platform *use* is
//! already prevented one layer up, structurally rather than by a banner: `🖥️host/📦️packages/🦀️rust/
//! Cargo.toml` only pulls this crate in under `[target.'cfg(target_os = "linux")'.dependencies]`, so no
//! consumer on macOS/Windows ever sees this crate's dependency edge, let alone its (absent) symbols —
//! referencing `VulkanBackend` from such a consumer fails with a plain "unresolved import", same category
//! of error a `compile_error!` banner would have produced, just raised at the actual misuse site instead
//! of unconditionally at this crate's own root. Same discipline applied to the D3D12 backend's
//! `📦️glue.rs`; see that file's header for the identical reasoning.
//!
//! See `🦀️backend.rs`'s header for which milestone this crate reaches and
//! `📓️terra-backend-vulkan-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

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
