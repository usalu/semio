//! 📦 Document archive contract of the app channel: the flat recursive-document schema, and its controlled decoding, split from `semio-os-channel` along the archive domain.

#[path = "🧬️schema/🦀️.rs"]
pub mod archive_schema;
#[path = "🛬️decode/🦀️.rs"]
pub mod archive_decoding;
#[path = "🧭️reopen-route/🦀️.rs"]
pub mod archive_reopen_route;
#[cfg(test)]
#[path="../../../🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🪙️close/🦀️.rs"]
pub(crate) mod original_close;
#[cfg(test)]
use semio_framework_value_testing_allocator as _;
