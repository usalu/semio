//! 🛫️ Lift file: 1 item(s), about 1 lines, of `🎞️media/🛫️intrinsic/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (media_projection_error); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use semio_framework_value::ValueError as MediaValueError;
use semio_framework_value::ValueRefusalKind as MediaRefusal;
pub fn media_projection_error() -> MediaValueError { MediaValueError::literal(MediaRefusal::InvariantViolated, "intrinsic media original source path changed") }
