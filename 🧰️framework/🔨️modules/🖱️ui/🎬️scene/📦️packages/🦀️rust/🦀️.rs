//! 🎬️ Product scene payloads for the semantic UI contract's `Component::Surface(SurfaceProps)` —
//! the 15 `SceneDoc` structs each embedded product surface (`world-3d`, `table`, `text-editor`, ...)
//! carries as an opaque pack-encoded `SurfaceDoc.bytes` blob, plus the generic 3D scene math shared
//! by every mesh/world viewport. Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet
//! `scene-surface`; absorbs the unfinished scene-crate item from ticket
//! 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY.
//!
//! Three properties define this crate:
//!
//! 1. **Wasm-safe by construction.** Depends on `ui_contract` and `serde` only (plus the tiny,
//!    equally dependency-free `semio-framework-geometry` for `math`'s `Mat4`/`Vec3`) — no OS, no
//!    GPU, no tokio, no `std::fs`. `cargo check --target wasm32-unknown-unknown` is a standing gate.
//! 2. **Product/renderer-neutral.** `🦀️scenes.rs`'s 15 structs and `🦀️math.rs`'s camera/mesh/picking
//!    math know nothing about `ui_wgpu`, React, or any specific renderer — `ui_wgpu` re-exports the
//!    former (the honest expression of "this crate uses that type", not a compatibility shim) and
//!    re-exports `math` under its old `kernel_3d_scene` name for its existing call sites.
//! 3. **Sync by decree (ruling E6).** Frame construction and payload encoding are plain `fn` here,
//!    same as `ui_contract`/`ui_runtime`/`ui_render` — async lives only at the outer boundaries.

#[path = "📐️math.rs"]
pub mod math;
#[path = "📦️pack.rs"]
pub mod pack;
#[path = "🎬️scenes.rs"]
mod scenes;
#[path = "🖼️canvas2d_snapshot.rs"]
mod canvas2d_snapshot;
#[path = "🌉️surface.rs"]
mod surface;
#[path = "🌍️world3d_snapshot.rs"]
mod world3d_snapshot;

pub use scenes::*;
pub use canvas2d_snapshot::*;
pub use surface::*;
pub use world3d_snapshot::*;
