//! 🖼 Raster identity and original pooled ownership independent of prepared rendering.
pub const RASTER_CONTENT_PAGE_BYTES: usize = 16 * 1024;
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::RasterContentIdentity;
#[path="📦️pool/🦀️.rs"]
mod pool;
pub use pool::*;
#[cfg(test)]
#[path="../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
