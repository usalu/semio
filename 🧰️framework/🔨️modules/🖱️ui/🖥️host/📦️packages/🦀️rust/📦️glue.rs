//! @emoji 🖥️ The platform layer: the only crate that knows about windows, event loops and which
//! graphics backend this target actually compiles.
//!
//! It is where the cfg-exclusive backend choice is resolved into a concrete [`ActiveBackend`] alias —
//! browser WebGPU on wasm, Metal on macOS, Direct3D 12 on Windows, Vulkan on Linux. Nothing anywhere
//! stores a `Box<dyn GraphicsBackend>`; a host or frame driver is generic over the backend type, so
//! retargeting a platform costs exactly one alias.
//!
//! Platform events are normalized here into the render crate's own multi-pointer vocabulary before
//! they reach dispatch, so no `winit` or `web_sys` type ever appears above this layer.
//!
//! Async lives here and only here: the outer event loop awaits, and device construction awaits. The
//! frame transaction it drives does not (ruling U1, ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`).

#[path = "🦀️backend_alias.rs"]
mod backend_alias;
#[path = "🦀️event.rs"]
mod event;
#[path = "🦀️window.rs"]
mod window;

pub use backend_alias::*;
pub use event::*;
pub use window::*;
