//! 🌪️ Block 3d vortices mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪧change-vortex-label/🦀️.rs"]
pub mod change_vortex_label;
#[path = "../🧷change-vortex-vortex-kind/🦀️.rs"]
pub mod change_vortex_vortex_kind;
#[path = "../🌀create-vortex/🦀️.rs"]
pub mod create_vortex;
#[path = "../🕳️delete-vortex/🦀️.rs"]
pub mod delete_vortex;
#[path = "../📍move-vortex/🦀️.rs"]
pub mod move_vortex;
#[path = "../📏resize-vortex/🦀️.rs"]
pub mod resize_vortex;
//#endregion 🪢️Mounts
