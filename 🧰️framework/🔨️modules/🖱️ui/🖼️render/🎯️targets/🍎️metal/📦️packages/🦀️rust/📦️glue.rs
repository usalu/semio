//! @emoji 🍎️ Hand-written Metal backend for macOS.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! See `🦀️backend.rs`'s header for which milestones this crate reaches and
//! `📓️terra-backend-metal-report.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) for
//! the authoritative status, decisions, and registrar-requests.

#[cfg(not(target_os = "macos"))]
compile_error!("semio-framework-ui-backend-metal builds only on macOS.");

#[path = "🦀️msl.rs"]
mod msl;
#[path = "🦀️types.rs"]
mod types;
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

pub use backend::{MetalBackend, MetalGraphicsError};
