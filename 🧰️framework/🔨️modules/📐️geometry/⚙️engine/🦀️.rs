//! 📐️ 2D geometry: single-source first-party Point/Vec2/Affine/shape primitives, selection math, and curve/polygon algorithms. `kurbo` is a test-only differential oracle.

// #region 🔖️Shapes
//! 📦️ First-party 2D geometry primitives — the one interface boundary the rest of the codebase depends on. Every guest-reachable type here is a plain value on every target, including `wasm32-wasip2`.

#[path = "🧩️shape-ref/🦀️.rs"]
mod shape_ref;
pub use shape_ref::*;

#[path = "📍️point-and-vector/🦀️.rs"]
mod point_and_vector;
pub use point_and_vector::*;

#[path = "🧮️affine/🦀️.rs"]
mod affine;
pub use affine::*;

#[path = "🔲️rect-shapes/🦀️.rs"]
mod rect_shapes;
pub use rect_shapes::*;

#[path = "⭕️curve-shapes/🦀️.rs"]
mod curve_shapes;
pub use curve_shapes::*;

#[path = "🛤️bez-path/🦀️.rs"]
mod bez_path;
pub use bez_path::*;

#[path = "✂️path-seg/🦀️.rs"]
mod path_seg;
pub use path_seg::*;

#[path = "🌓️arc-flattening-and-path-append/🦀️.rs"]
mod arc_flattening_and_path_append;
pub use arc_flattening_and_path_append::*;

#[path = "📏️generic-geometry/🦀️.rs"]
mod generic_geometry;
pub use generic_geometry::*;

#[path = "📈️curve-extensions/🦀️.rs"]
mod curve_extensions;
pub use curve_extensions::*;

#[path = "🔺️polygon-extensions/🦀️.rs"]
mod polygon_extensions;
pub use polygon_extensions::*;

#[path = "🧭️vec3/🦀️.rs"]
mod vec3;
pub use vec3::*;

#[path = "🧱️mat4/🦀️.rs"]
mod mat4;
pub use mat4::*;

// #endregion 🔖️Shapes

// #region 🔖️GeomSel
#[path = "🎯️world-selection/🦀️.rs"]
pub mod geom_sel;
// #endregion 🔖️GeomSel

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️PathSegTests
#[cfg(test)]
#[path = "🧪️tests/🔬️path-seg/🦀️.rs"]
mod path_seg_tests;
// #endregion 🔖️PathSegTests

// #region 🔖️FirstPartyShapeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️first-party-shape/🦀️.rs"]
mod first_party_shape_tests;
// #endregion 🔖️FirstPartyShapeTests

// #region 🔖️AlgebraTests
#[cfg(test)]
#[path = "🧪️tests/🔬️algebra/🦀️.rs"]
mod algebra_tests;
// #endregion 🔖️AlgebraTests
