//! 🔬️ The probe artifact document: JSON and SQLite snapshot codecs.

extern crate semio_os_kernel as pack;
extern crate semio_os_kernel as store;

#[path = "../🏠️workspace/🔬️probe-document/🦀️.rs"]
pub mod probe_document;

#[path = "../🏠️workspace/🚪️io/🔤️json/🦀️.rs"]
pub mod probe_json;

#[path = "../🏠️workspace/🪶️sqlite/🦀️.rs"]
pub mod probe_sqlite;

// [DEBUG] #[cfg(test)]
// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🫴️native/🧪️tests/🦀️.rs"]
// [DEBUG] pub mod probe_sqlite_original_io;

// [DEBUG] #[cfg(test)]
// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🫴️semantic/🧪️tests/🦀️.rs"]
// [DEBUG] pub mod probe_sqlite_original_semantic;

// [DEBUG] #[cfg(test)]
// [DEBUG] #[path = "../🏠️workspace/🪶️sqlite/🧪️tests/🦀️.rs"]
// [DEBUG] pub mod probe_sqlite_tests;

#[path = "../🏠️workspace/🪶️sqlite/🫴️semantic/🦀️.rs"]
pub mod semantic;
