//! 🧫️ Sqlite snapshot laws of the semio root artifact for the subsets mesh, presentation, document, text: the original test files, mounted unchanged, and the `fixture` of each subset that the base union law reuses.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
#![allow(dead_code, unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub(crate) use semio_framework_io_sqlite_snapshot_testing_backing as snapshot_test_backing;
pub use semio_s_artifact_stdio_semio::*;

#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod document;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod presentation;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod text;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod mesh;
