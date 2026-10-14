//! 📚️ Block 3d example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🌲️hexagonal-cut-concrete-forest-left/🦀️.rs"]
pub mod art_3d_hexagonal_cut_concrete_forest_left;
#[cfg(test)]
#[path = "🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🦀️.rs"]
mod art_3d_hexagonal_cut_concrete_forest_left_tests;
#[path = "🏢️nakagin-capsule/🦀️.rs"]
pub mod art_3d_nakagin_capsule;
#[cfg(test)]
#[path = "🏢️nakagin-capsule/🧪️tests/🧩️example/🦀️.rs"]
mod art_3d_nakagin_capsule_tests;
//#endregion 🪢️Mounts
