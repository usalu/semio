//! 📚️ Block 2d example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🌲️hexagonal-cut-concrete-forest-left/🦀️.rs"]
pub mod art_2d_hexagonal_cut_concrete_forest_left;
#[cfg(test)]
#[path = "🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🦀️.rs"]
mod art_2d_hexagonal_cut_concrete_forest_left_tests;
#[path = "➡️hexagonal-cut-concrete-forest-right/🦀️.rs"]
pub mod art_2d_hexagonal_cut_concrete_forest_right;
#[cfg(test)]
#[path = "➡️hexagonal-cut-concrete-forest-right/🧪️tests/🧩️example/🦀️.rs"]
mod art_2d_hexagonal_cut_concrete_forest_right_tests;
//#endregion 🪢️Mounts
