//! @emoji 🪟️ Hand-written Direct3D 12 backend for Windows.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! **Every module below is `target_os = "windows"`-gated.** `Cargo.toml` puts `windows`/
//! `raw-window-handle` behind `[target.'cfg(target_os = "windows")'.dependencies]`, so on any other host
//! those crates are not even in the dependency graph — ungated `mod` declarations would still try to
//! resolve `use windows::…;` and fail with cascading "can't find crate" errors.
//!
//! **On a non-Windows host this crate compiles to an empty, zero-item lib — deliberately, not an
//! oversight.** It used to gate on a hard `compile_error!` instead, but that made `cargo check
//! --workspace` (this refactor's exit gate) fail on every non-Windows host merely because this crate is
//! a workspace member, independent of whether anything actually depends on it. Wrong-platform *use* is
//! already prevented one layer up, structurally rather than by a banner: `🖥️host/📦️packages/🦀️rust/
//! Cargo.toml` only pulls this crate in under `[target.'cfg(target_os = "windows")'.dependencies]`, so
//! no consumer on macOS/Linux ever sees this crate's dependency edge, let alone its (absent) symbols —
//! referencing `D3d12Backend` from such a consumer fails with a plain "unresolved import", same category
//! of error a `compile_error!` banner would have produced, just raised at the actual misuse site instead
//! of unconditionally at this crate's own root. Same discipline applied to the Vulkan backend's
//! `🦀️.rs`; see that file's header for the identical reasoning.
//!
//! See `🦀️backend.rs`'s header for which milestones this crate reaches and
//! `📓️terra-backend-d3d12-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

#[cfg(target_os = "windows")]
#[path = "🦀️backend.rs"]
mod backend;
#[cfg(target_os = "windows")]
#[path = "🦀️frame_buffers.rs"]
mod frame_buffers;
#[cfg(target_os = "windows")]
#[path = "🦀️hlsl.rs"]
mod hlsl;
#[cfg(target_os = "windows")]
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[cfg(target_os = "windows")]
#[path = "🦀️resources.rs"]
mod resources;
#[cfg(target_os = "windows")]
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[cfg(target_os = "windows")]
#[path = "🦀️types.rs"]
mod types;
#[cfg(target_os = "windows")]
#[path = "🦀️world3d.rs"]
mod world3d;

#[cfg(target_os = "windows")]
pub use backend::D3d12Backend;
