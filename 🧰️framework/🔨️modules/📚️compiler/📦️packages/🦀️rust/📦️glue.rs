//! 📚️ The semio incremental document compiler: syntax, world, text, math layout, and SVG targets in one crate.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

pub use dsl_core::os_dsl;

#[path = "../../📖️syntax/🦀️component.rs"]
pub mod syntax;

#[path = "../../🌍️world/🦀️component.rs"]
pub mod world;

// 🔤️ Real glyph shaping/outline/math-table access (`rustybuzz`) and SVG serialization (`base64`
// for embedded raster glyphs) — host/browser-only. See this crate's Cargo.toml docstring and
// `🦀️component.rs`'s `estimate_svg` for the `wasm32-wasip2` counterpart.
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../🔤️text/🦀️component.rs"]
pub mod text;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../🧮️math/🦀️component.rs"]
pub mod math;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../📤️svg/🦀️component.rs"]
pub mod svg;

#[path = "../../🦀️component.rs"]
mod facade;

pub use facade::*;
