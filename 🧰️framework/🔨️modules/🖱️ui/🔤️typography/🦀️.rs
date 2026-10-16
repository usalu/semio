//! 🔤️ Renderer-independent font metrics, glyph rasterization and own text records.
#[path="🧩️component/🦀️.rs"]
pub mod text;
pub use text::*;
#[path="../🚪️io/🔤️font/🤝️pair-adjustment/🦀️.rs"]
pub mod font_pair_io;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
