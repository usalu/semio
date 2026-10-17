//! 🖌️ The first-party scene: the paint value types, the recorded command list `Scene`, its retirement registry, the portable draw list and the parsed SVG document.

// #region 🏷️VelloBackend
pub(super) mod vello_backend {
    /// 🖊️ `vello`'s font/SVG/codec transitive tail (`vello_encoding`, `skrifa`, `read-fonts`,
    /// `font-types`, `png`, `guillotiere`, `moxcms`, `pxfm`, …) is only needed where a real
    /// `vello::Scene` is actually rasterized — the host/browser target table. `Scene` itself
    /// is a first-party command list (see `SceneCommand` below). `kurbo`/`peniko` (the
    /// value-type family `Stroke`/`Color`/`Fill`/`Mix`/`ImageData` are built on) are ALSO
    /// host/browser-only now: every guest-reachable value type (`Cap`, `Join`, `Stroke`,
    /// `Color`, `FillRule`, `BlendMode`, `RasterImage`) is first-party, converting to a real
    /// `kurbo`/`peniko` value only inside `SceneCommand::replay_into` — the same
    /// already-host-gated boundary `vello` itself crosses at. `target_arch = "wasm32"` alone
    /// is TRUE for `wasm32-wasip2`, so the gate is always the
    /// `not(all(target_arch = "wasm32", target_env = "p2"))` compound. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
    /// `🔍️research/📓️kurbo-peniko-first-party.md` (extends
    /// `🔍️research/📓️vello-scene-first-party.md`).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use vello::util;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use vello::wgpu;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello::Scene;
    /// 🎨️ `usvg` (via `vello_svg`) is a rendering/text-shaping dependency: every real
    /// consumer — `svg_icon`'s paint pipeline, `SvgDocument`, `mod text`'s label shaper — is
    /// reachable only from the host/browser paint tree or from `semio-framework-editor`
    /// (itself absent from every plugin's `wasm32-wasip2` dependency graph), never from a
    /// WASI guest. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
    /// ARTIFACTS`, `🔍️research/📓️infinite-text-shaping.md`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello_svg;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello_svg::usvg;
}
// #endregion 🏷️VelloBackend

use geometry::{Affine, Arc, BezPath, Circle, CubicBez, Line, Rect, RoundedRect, ShapeRef};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use geometry::{PathEl, Point};
use std::mem::{size_of, ManuallyDrop};
use std::sync::{Arc as SharedArc, Mutex, OnceLock};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use vello_backend as backend;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "🔧️kurbo-boundary/🦀️.rs"]
mod kurbo_boundary;
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
use kurbo_boundary::*;

#[path = "🖊️stroke-style/🦀️.rs"]
mod stroke_style;
pub use stroke_style::*;

#[path = "🎨️paint-values/🦀️.rs"]
mod paint_values;
pub use paint_values::*;

#[path = "📦️shape-and-command/🦀️.rs"]
mod shape_and_command;
pub(crate) use shape_and_command::*;

#[path = "🎬️scene-and-retirement/🦀️.rs"]
mod scene_and_retirement;
pub use scene_and_retirement::*;

/// 🧪️ Differential proof that the first-party `Color`/`Stroke`/`RasterImage` reimplementations
/// agree with the real `kurbo`/`peniko` (`color`) types they replaced on `wasm32-wasip2` — the
/// oracle is used directly (this crate already depends on `kurbo`/`peniko` on the host target
/// these tests run on, via the boundary conversions above), same convention as
/// `semio-framework-geometry`'s `path_seg_tests`.
#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../../🧪️tests/🔬️renderer-color-stroke-raster/🦀️.rs");

// #region 🔖️DrawList
/// 🎬️ Portable replay encoding of a first-party [`Scene`] — the SAME command list
/// [`Scene::vello_scene`] rasterizes on a GPU target, written out as numbers a renderer with
/// no `vello` (a browser 2D canvas context, a conformance twin in any language) can replay
/// verb for verb. It exists because a `Scene` painted inside a wasm guest has, until now, had
/// exactly one exit: a real GPU device. Where there is none, the picture was dropped on the
/// floor and the host invented its own.
///
/// Shapes stay PRIMITIVES (`rect`/`rrect`/`circle`/`line`/`cubic`) instead of being flattened
/// to polylines, so a replaying 2D context reaches its own `roundRect`/`arc`/`bezierCurveTo`
/// and a later zoom cannot facet a circle — the same reason [`RecordedShape`] keeps them
/// exact. Only `Arc` and `BezPath` become verb streams, because no 2D primitive matches them.
///
/// Every command carries its own affine, and [`DagHost::paint_scene`] already bakes the camera
/// into those affines (`camera_content_affine`), so a replay is camera-correct with no second
/// transform pipeline. Device-pixel scaling is deliberately NOT baked in (unlike
/// [`render::scale_scene_for_device_pixel_ratio`], which the GPU path needs): the replayer sets
/// its own base transform, which keeps a draw list a pure function of scene + camera and so
/// directly comparable across implementations.
#[path = "🗒️draw-list/🦀️.rs"]
pub mod draw_list;
// #endregion 🔖️DrawList

#[cfg(test)]
include!("../../🧪️tests/🔬️renderer-opaque-scene-retirement/🦀️.rs");
