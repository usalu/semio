//! 🎭️ Block 5d representations mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧩add-representation-attribute/🦀️.rs"]
pub mod add_representation_attribute;
#[path = "../🔖add-representation-tag/🦀️.rs"]
pub mod add_representation_tag;
#[path = "../📜change-representation/🦀️.rs"]
pub mod change_representation_description;
#[path = "../🏔️change-representation-lod/🦀️.rs"]
pub mod change_representation_lod;
#[path = "../🌐change-representation-mesh-url/🦀️.rs"]
pub mod change_representation_mesh_url;
#[path = "../🧱create-representation/🦀️.rs"]
pub mod create_representation;
#[path = "../🗑️delete-representation/🦀️.rs"]
pub mod delete_representation;
#[path = "../➖remove-representation-attribute/🦀️.rs"]
pub mod remove_representation_attribute;
#[path = "../🚫remove-representation-tag/🦀️.rs"]
pub mod remove_representation_tag;
#[path = "../✒️rename-representation/🦀️.rs"]
pub mod rename_representation;
//#endregion 🪢️Mounts
