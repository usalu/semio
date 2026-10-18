//! 🔬️ The probe artifact document: JSON and SQLite snapshot codecs.

extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

#[path = "../🏠️workspace/🔬️probe-document/🦀️.rs"]
pub mod probe_document;

#[path = "../🏠️workspace/🚪️io/🔤️json/🦀️.rs"]
pub mod probe_json;

#[path = "../🏠️workspace/🪶️sqlite/🦀️.rs"]
pub mod probe_sqlite;

#[cfg(test)]
#[path = "../🏠️workspace/🪶️sqlite/🫴️native/🧪️tests/🦀️.rs"]
pub mod probe_sqlite_original_io;

#[cfg(test)]
#[path = "../🏠️workspace/🪶️sqlite/🫴️semantic/🧪️tests/🦀️.rs"]
pub mod probe_sqlite_original_semantic;

#[cfg(test)]
#[path = "../🏠️workspace/🪶️sqlite/🧪️tests/🦀️.rs"]
pub mod probe_sqlite_tests;

#[path = "../🏠️workspace/🪶️sqlite/🫴️semantic/🦀️.rs"]
pub mod semantic;
