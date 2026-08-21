//! @emoji 🖼️ The custom GPU renderer's backend-neutral core.
//!
//! This crate lowers a semantic [`ui_contract::UiSnapshot`] into pixels' worth of *description* — a
//! [`RenderPacket`] — and stops there. It contains no device, no swapchain, no shader compilation and
//! no window: `wgpu`, `winit` and every graphics binding live in the backend and host crates, and a CI
//! `cargo tree` assertion fails the build if one of them appears here.
//!
//! The pipeline is one synchronous run-to-completion transaction (ruling U1, ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`):
//!
//! ```text
//! UiSnapshot/UiPatch → elements → request_layout → taffy → prepaint → paint
//!                    → Scene::finish → FrameSnapshot → RenderPacket → GraphicsBackend
//! ```
//!
//! Nothing suspends inside it. A dependency that is not ready yet (an unshaped font, an undecoded
//! image) is represented as `Measurement::Pending` and drawn as a placeholder; when it lands, the
//! resource registry invalidates the windows that referenced it and a *later* frame shows it. That is
//! why input dispatch can be guaranteed to run against the same generation the user actually saw:
//! `FrameSnapshot` carries the scene, hitboxes, dispatch tree, focus, IME and accessibility together,
//! and is swapped in atomically.

#[path = "🦀️backend.rs"]
mod backend;
#[path = "🦀️dispatch.rs"]
mod dispatch;
#[path = "🦀️element.rs"]
mod element;
#[path = "🦀️frame.rs"]
mod frame;
#[path = "🦀️layout.rs"]
mod layout;
#[path = "🦀️resource.rs"]
mod resource;
#[path = "🦀️scene.rs"]
mod scene;
#[path = "🦀️schedule.rs"]
mod schedule;
#[path = "🦀️shader_contract.rs"]
mod shader_contract;
#[path = "🦀️surface.rs"]
mod surface;
#[path = "🦀️tessellate.rs"]
mod tessellate;
#[path = "🦀️text.rs"]
mod text;

pub use backend::*;
pub use dispatch::*;
pub use element::*;
pub use frame::*;
pub use layout::*;
pub use resource::*;
pub use scene::*;
pub use schedule::*;
pub use shader_contract::*;
pub use surface::*;
pub use tessellate::*;
pub use text::*;
