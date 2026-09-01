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
//! Device-shaped modules below are `all(target_arch = "wasm32", not(target_env = "p2"))`-gated.
//! `Cargo.toml` puts `wgpu` behind the same gate, so native hosts never resolve it. The narrower
//! form (not the bare `target_arch = "wasm32"` this crate used before) matters because
//! `target_arch = "wasm32"` is also TRUE for the `wasm32-wasip2` component target, and `wgpu`'s
//! `webgpu` feature pulls `wasm-bindgen`/`js-sys`/`web-sys` — browser bindings with no meaning in a
//! WASI component. The owned byte/page surface adapter remains target-neutral so its lifecycle laws
//! run without a browser.
//!
//! On a non-browser target only the dependency-free contract, codec, admission ledger, and state
//! machine compile. Device resources remain absent, so referencing `WebGpuBackend` from a native
//! consumer still fails at the misuse site while native tests can execute the complete surface ABI.

//#region 🔖️Backend

#[path = "../../../../../../🌉️abi/🦀️component.rs"]
pub mod abi;
#[path = "🦀️surface_adapter.rs"]
mod surface_adapter;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️backend.rs"]
mod backend;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️buffers.rs"]
mod buffers;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️frame.rs"]
mod frame;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️gpu_context.rs"]
mod gpu_context;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️gpu_types.rs"]
mod gpu_types;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️gpu_uniforms.rs"]
mod gpu_uniforms;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️resources.rs"]
mod resources;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🦀️surface_state.rs"]
mod surface_state;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use backend::WebGpuBackend;
pub use surface_adapter::*;

//#endregion 🔖️Backend
