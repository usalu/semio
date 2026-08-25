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
//! Device-shaped modules below are `target_arch = "wasm32"`-gated. `Cargo.toml` puts `wgpu` behind
//! `[target.'cfg(target_arch = "wasm32")'.dependencies]`, so native hosts never resolve it. The owned
//! byte/page surface adapter remains target-neutral so its lifecycle laws run without a browser.
//!
//! On a non-browser target only the dependency-free contract, codec, admission ledger, and state
//! machine compile. Device resources remain absent, so referencing `WebGpuBackend` from a native
//! consumer still fails at the misuse site while native tests can execute the complete surface ABI.

//#region 🔖️Backend

#[path = "../../../../../../🌉️abi/🦀️component.rs"]
pub mod abi;
#[path = "🦀️surface_adapter.rs"]
mod surface_adapter;

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
pub use surface_adapter::*;

//#endregion 🔖️Backend
