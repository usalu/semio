//! 📚️ The semio incremental document compiler: syntax, world, text, math layout, and SVG targets in one crate.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

pub use dsl_core::os_dsl;

#[path = "../../📖️syntax/🦀️component.rs"]
pub mod syntax;

#[path = "../../🌍️world/🦀️component.rs"]
pub mod world;

#[path = "../../🔤️text/🦀️component.rs"]
pub mod text;

#[path = "../../🧮️math/🦀️component.rs"]
pub mod math;

#[path = "../../📤️svg/🦀️component.rs"]
pub mod svg;

#[path = "../../🦀️component.rs"]
mod facade;

pub use facade::*;
