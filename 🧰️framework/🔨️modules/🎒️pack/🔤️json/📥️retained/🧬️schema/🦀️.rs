//! 🔣️ Fixed-page retained JSON token, record, string and hexadecimal schema authority.
use semio_framework_value::{ValueError, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::{size_of, size_of_val};
#[path="../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/⚠️diagnostic/🦀️.rs"]
pub mod schema_diagnostic;

#[path = "📃️decode-pages/🦀️.rs"]
mod decode_pages;
pub use decode_pages::*;

#[path = "🔡️path-and-token-cursor/🦀️.rs"]
mod path_and_token_cursor;
pub use path_and_token_cursor::*;

#[path = "🗂️record-cursors/🦀️.rs"]
mod record_cursors;
pub use record_cursors::*;

#[path = "🔐️string-and-hex-authorities/🦀️.rs"]
mod string_and_hex_authorities;
pub use string_and_hex_authorities::*;

#[cfg(any(test, feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod testing;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🦀️.rs"]
mod owned_schema_record_tests;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
