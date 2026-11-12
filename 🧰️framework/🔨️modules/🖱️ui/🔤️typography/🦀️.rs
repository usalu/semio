//! 🔤️ Renderer-independent font metrics, glyph rasterization and own text records.
#[path="🧩️component/🦀️.rs"]
pub mod text;
pub use text::*;
#[path="../🚪️io/🔤️font/🤝️pair-adjustment/🦀️.rs"]
pub mod font_pair_io;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
