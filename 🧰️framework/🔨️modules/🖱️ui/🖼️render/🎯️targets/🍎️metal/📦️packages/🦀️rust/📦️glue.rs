//! @emoji 🍎️ Hand-written Metal backend for macOS.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! **Every module below is `target_os = "macos"`-gated.** `Cargo.toml` puts `objc2`/`objc2-metal`/
//! `objc2-foundation` (and friends) behind `[target.'cfg(target_os = "macos")'.dependencies]`, so on any
//! other host those crates are not even in the dependency graph — ungated `mod` declarations would still
//! try to resolve `use objc2::…;` and fail with cascading "can't find crate" errors.
//!
//! **On a non-macOS host this crate compiles to an empty, zero-item lib — deliberately, not an
//! oversight.** It used to gate on a hard `compile_error!` instead, but that made `cargo check
//! --workspace` (this refactor's exit gate) fail on every non-macOS host merely because this crate is a
//! workspace member, independent of whether anything actually depends on it. Wrong-platform *use* is
//! already prevented one layer up, structurally rather than by a banner: `🖥️host/📦️packages/🦀️rust/
//! Cargo.toml` only pulls this crate in under `[target.'cfg(target_os = "macos")'.dependencies]`, so no
//! consumer on Linux/Windows ever sees this crate's dependency edge, let alone its (absent) symbols —
//! referencing `MetalBackend` from such a consumer fails with a plain "unresolved import", same category
//! of error a `compile_error!` banner would have produced, just raised at the actual misuse site instead
//! of unconditionally at this crate's own root. Same discipline applied to the Vulkan/D3D12/WebGPU
//! backends' `📦️glue.rs`; see those files' headers for the identical reasoning.
//!
//! See `🦀️backend.rs`'s header for which milestones this crate reaches and
//! `📓️terra-backend-metal-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

#[cfg(target_os = "macos")]
#[path = "🦀️msl.rs"]
mod msl;
#[cfg(target_os = "macos")]
#[path = "🦀️types.rs"]
mod types;
#[cfg(target_os = "macos")]
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[cfg(target_os = "macos")]
#[path = "🦀️resources.rs"]
mod resources;
#[cfg(target_os = "macos")]
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[cfg(target_os = "macos")]
#[path = "🦀️frame_buffers.rs"]
mod frame_buffers;
#[cfg(target_os = "macos")]
#[path = "🦀️world3d.rs"]
mod world3d;
#[cfg(target_os = "macos")]
#[path = "🦀️backend.rs"]
mod backend;

#[cfg(target_os = "macos")]
pub use backend::{MetalBackend, MetalGraphicsError};
