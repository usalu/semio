//! 🧫️ Sqlite snapshot laws of the semio root artifact for the subsets flow, drawing, cad, audio: the original test files, mounted unchanged, and the `fixture` of each subset that the base union law reuses.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
#![allow(dead_code, unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_semio::*;

#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod flow;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod cad;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod audio;
#[path = "../../../../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod drawing;
