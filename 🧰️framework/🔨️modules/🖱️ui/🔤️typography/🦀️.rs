//! 🔤️ Renderer-independent font metrics, glyph rasterization and own text records.
#[path="🧩️component/🦀️.rs"]
pub mod text;
pub use text::*;
#[path="../🚪️io/🔤️font/🤝️pair-adjustment/🦀️.rs"]
pub mod font_pair_io;

#[cfg(test)]
#[path="../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
