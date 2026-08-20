//! @emoji 🌐️ Browser WebGPU backend — the one sanctioned home of `wgpu` in this repo.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! ⚠️ SCAFFOLD — owned by packet `backend-webgpu`. Replace this placeholder wholesale.

#[cfg(not(target_arch = "wasm32"))]
compile_error!("semio-framework-ui-backend-webgpu is browser-only: wgpu is deliberately confined to wasm32 builds. Native targets use the hand-written metal/d3d12/vulkan backends.");

//#region 🔖️Backend

#[path = "🦀️surface_state.rs"]
mod surface_state;
#[path = "🦀️gpu_types.rs"]
mod gpu_types;
#[path = "🦀️gpu_uniforms.rs"]
mod gpu_uniforms;
#[path = "🦀️gpu_context.rs"]
mod gpu_context;
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[path = "🦀️resources.rs"]
mod resources;
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[path = "🦀️buffers.rs"]
mod buffers;
#[path = "🦀️frame.rs"]
mod frame;
#[path = "🦀️backend.rs"]
mod backend;

pub use backend::WebGpuBackend;

//#endregion 🔖️Backend
