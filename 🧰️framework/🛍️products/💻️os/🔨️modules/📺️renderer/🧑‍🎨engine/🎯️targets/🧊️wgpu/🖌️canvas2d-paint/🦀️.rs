//! 🖌️ Path geometry and paint of the Canvas2d host.
//!
//! 🧭️ Native and wasm32, no first-party dependencies. The module file stays in the `📐️Canvas2dHost` element tree (a WP1 crate whose one source sits in WP2's folder); this crate root mounts it and re-exports its surface.

#[path = "../../../🧱️elements/📐️Canvas2dHost/🎨️paint/🦀️.rs"]
mod canvas2d_paint;
pub use canvas2d_paint::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
