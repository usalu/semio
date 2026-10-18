//! 🧫️ Sqlite snapshot laws of the semio root artifact for the subsets model, kit, table, value: the original test files, mounted unchanged, and the `fixture` of each subset that the base union law reuses.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
#![allow(dead_code, unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_semio::*;

#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod model;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod table;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod value;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod kit;
