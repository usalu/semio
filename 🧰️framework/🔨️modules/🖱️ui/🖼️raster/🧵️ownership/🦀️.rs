//! 🖼 Raster identity and original pooled ownership independent of prepared rendering.
pub const RASTER_CONTENT_PAGE_BYTES: usize = 16 * 1024;
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::RasterContentIdentity;
#[path="📦️pool/🦀️.rs"]
mod pool;
pub use pool::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as _;
