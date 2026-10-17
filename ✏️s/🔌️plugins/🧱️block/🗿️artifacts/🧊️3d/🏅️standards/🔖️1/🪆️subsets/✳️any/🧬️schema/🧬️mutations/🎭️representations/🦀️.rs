//! 🎭️ Block 3d representations mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧩add-representation-attribute/🦀️.rs"]
pub mod add_representation_attribute;
#[path = "../🌐change-representation-mesh-url/🦀️.rs"]
pub mod change_representation_mesh_url;
#[path = "../🧱create-representation/🦀️.rs"]
pub mod create_representation;
#[path = "../🗑️delete-representation/🦀️.rs"]
pub mod delete_representation;
#[path = "../➖remove-representation-attribute/🦀️.rs"]
pub mod remove_representation_attribute;
//#endregion 🪢️Mounts

pub use semio_s_artifact_block_3d_mut_representations_2::{add_representation_tag, change_representation_description, change_representation_lod, remove_representation_tag, rename_representation};
