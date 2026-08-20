//! @emoji 🪟️ Hand-written Direct3D 12 backend for Windows.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! See `🦀️backend.rs`'s header for which milestones this crate reaches and
//! `📓️terra-backend-d3d12-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

#[cfg(not(target_os = "windows"))]
compile_error!("semio-framework-ui-backend-d3d12 builds only on Windows.");

#[path = "🦀️types.rs"]
mod types;
#[path = "🦀️hlsl.rs"]
mod hlsl;
#[path = "🦀️pipelines.rs"]
mod pipelines;
#[path = "🦀️resources.rs"]
mod resources;
#[path = "🦀️scene_target.rs"]
mod scene_target;
#[path = "🦀️frame_buffers.rs"]
mod frame_buffers;
#[path = "🦀️world3d.rs"]
mod world3d;
#[path = "🦀️backend.rs"]
mod backend;

pub use backend::D3d12Backend;
