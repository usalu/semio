//! 🪜️ Block 3d representations mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔖add-representation-tag/🦀️.rs"]
pub mod add_representation_tag;
#[path = "../📜change-representation/🦀️.rs"]
pub mod change_representation_description;
#[path = "../🏔️change-representation-lod/🦀️.rs"]
pub mod change_representation_lod;
#[path = "../🚫remove-representation-tag/🦀️.rs"]
pub mod remove_representation_tag;
#[path = "../✒️rename-representation/🦀️.rs"]
pub mod rename_representation;
//#endregion 🪢️Mounts
