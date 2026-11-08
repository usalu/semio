//! 🧳️ Native retained clone backing shares one original allocation and its genuine cursor lifecycle.
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_native_cursor_backing::{NativeProjectionBacking,NativeSeedTemplate};
#[path="../../🦀️.rs"]
mod facade;
pub use facade::{NativeRetainedCloneBacking,NativeRetainedCloneState};
