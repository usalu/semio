//! @emoji 🌐️ Browser WebGPU backend — the one sanctioned home of `wgpu` in this repo.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! ⚠️ SCAFFOLD — owned by packet `backend-webgpu`. Replace this placeholder wholesale.
//!
//! **Every module below is `target_arch = "wasm32"`-gated.** `Cargo.toml` puts `wgpu`/`web-sys`/
//! `wasm-bindgen` behind `[target.'cfg(target_arch = "wasm32")'.dependencies]`, so on any native host
//! those crates are not even in the dependency graph — ungated `mod` declarations would still try to
//! resolve `use wgpu::…;`/`use web_sys::…;` and fail with cascading "can't find crate" errors.
//!
//! **On a non-wasm32 host this crate compiles to an empty, zero-item lib — deliberately, not an
//! oversight.** It used to gate on a hard `compile_error!` instead, but that made `cargo check
//! --workspace` (this refactor's exit gate) fail on every native host merely because this crate is a
//! workspace member, independent of whether anything actually depends on it. Wrong-platform *use* is
//! already prevented one layer up, structurally rather than by a banner: `🖥️host/📦️packages/🦀️rust/
//! Cargo.toml` only pulls this crate in under `[target.'cfg(target_arch = "wasm32")'.dependencies]`, so
//! no consumer on macOS/Linux/Windows ever sees this crate's dependency edge, let alone its (absent)
//! symbols — referencing `WebGpuBackend` from such a consumer fails with a plain "unresolved import",
//! same category of error a `compile_error!` banner would have produced, just raised at the actual
//! misuse site instead of unconditionally at this crate's own root. Same discipline applied to the
//! Vulkan/D3D12/Metal backends' `📦️glue.rs`; see those files' headers for the identical reasoning.

//#region 🔖️Backend

#[cfg(target_arch = "wasm32")]
#[path = "🦀️backend.rs"]
mod backend;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️buffers.rs"]
mod buffers;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️frame.rs"]
mod frame;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️gpu_context.rs"]
mod gpu_context;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️gpu_types.rs"]
mod gpu_types;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️gpu_uniforms.rs"]
mod gpu_uniforms;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️resources.rs"]
mod resources;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[cfg(target_arch = "wasm32")]
#[path = "🦀️surface_state.rs"]
mod surface_state;

#[cfg(target_arch = "wasm32")]
pub use backend::WebGpuBackend;

//#endregion 🔖️Backend
