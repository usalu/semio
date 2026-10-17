//! 🧫️ Sqlite snapshot laws of the semio root artifact for the subsets animation, object, video: the original test files, mounted unchanged, and the `fixture` of each subset that the base union law reuses.

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

#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod animation;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod video;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod object;
