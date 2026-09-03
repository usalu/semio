//! ⚙️ 2D geometry kernel primitives: path segments and the shared point/error vocabulary consumed
//! by the [`crate::booleans`]/[`crate::trace`] pure-function kernels and by any external caller
//! (e.g. the `🖍️draw` plugin) that needs planar-boolean or bitmap-autotrace geometry.
//!
//! 🪦 The store-specific scene-graph vocabulary (`DrawingKernel`, `DrawingHandle`, `DrawingKind`,
//! `DrawingNode`, `SceneNode`, `DrawingScene`, `FillStyle`, `StrokeStyle`, `GradientStop`,
//! `LineCap`, `LineJoin`, `Affine2D`) relocated to the OS flow module's own drawing kernel
//! (`💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs`, ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) — its only two real
//! consumers are flow's own ephemeral node-evaluation kernel and the flow `draw` extension, neither
//! of which is the persisted-artifact surface `✳️drawing`'s real `ArtifactStore` + 17 mutation
//! triads + `🎛flattened-scene` inference now own. `PathSegment`/`Vec2` stay here because they are
//! a genuinely generic geometry-kernel working type shared by `booleans`/`trace` and unrelated
//! plugins (e.g. `🖍️draw`'s own boolean/trace bridging), not the drawing artifact's own schema.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

// #region 🔖️Types
/// 📐️ Column vector `[x,y]`.
pub type Vec2 = [f64; 2];

/// ✏️ Path segment in local coordinates.
// 🧬️ `#[derive(ToValue, FromValue)]` additive alongside `serde`
// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01) — `serde` is now behind
// this crate's off-by-default `serde` feature: `semio-framework-os-flow`'s `🖍️drawing/🦀️.rs`
// (`DrawingNode`, `SceneNode`, and a third private struct, none of them `#[cfg(test)]`) derives
// `serde::Serialize`/`Deserialize` over `Vec<semio_framework_2d::PathSegment>`/
// `Option<Vec<...>>` fields and enables the feature on its own `semio-framework-2d` dependency
// (confirmed: without it, `cargo check -p semio-framework-os-flow` → 15× E0277 `PathSegment:
// Serialize`/`Deserialize` not satisfied). No in-component crate (`semio-s-plugin-draw`,
// `semio-s-plugin-flow-extension-draw`) enables it — both convert to/from their own
// `ToValue`/`FromValue`-only wire types or only touch `DrawingError`/`Vec2` (neither serde-derived),
// so the feature stays off for every plugin wasm component. `#[value(tag = "kind", rename_all =
// "camelCase")]` mirrors `#[serde(tag = "kind", rename_all = "camelCase")]` exactly — internally
// tagged, and with no `rename_all_fields` given, the single case covers both the tag values AND
// each named-field variant's own field names (serde's own default when only `rename_all` is
// given), so `large_arc` renders as `largeArc` either way.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    Move { to: Vec2 },
    Line { to: Vec2 },
    Quad { ctrl: Vec2, to: Vec2 },
    Cubic { ctrl1: Vec2, ctrl2: Vec2, to: Vec2 },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: Vec2 },
    Close,
}

// #region ⚠️ Errors
/// ⚠️ Geometry-kernel operation error.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawingError {
    InvalidInput(String),
    MissingHandle(String),
    Operation(String),
}

impl fmt::Display for DrawingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
            Self::MissingHandle(handle) => write!(formatter, "missing handle: {handle}"),
            Self::Operation(message) => write!(formatter, "operation failed: {message}"),
        }
    }
}

impl std::error::Error for DrawingError {}
// #endregion ⚠️ Errors
// #endregion 🔖️Types
