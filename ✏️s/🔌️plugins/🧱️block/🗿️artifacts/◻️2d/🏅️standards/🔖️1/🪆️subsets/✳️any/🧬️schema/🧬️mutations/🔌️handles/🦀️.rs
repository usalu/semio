//! 🔌️ Block 2d handles mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧷️change-handle-handle-kind/🦀️.rs"]
pub mod change_handle_handle_kind;
#[path = "../🔌️change-handle-kind-default-wire-kind/🦀️.rs"]
pub mod change_handle_kind_default_wire_kind;
#[path = "../🌿️create-handle/🦀️.rs"]
pub mod create_handle;
#[path = "../❌️delete-handle/🦀️.rs"]
pub mod delete_handle;
#[path = "../📍️move-handle/🦀️.rs"]
pub mod move_handle;
//#endregion 🪢️Mounts

pub use semio_s_artifact_block_2d_mut_handles_2::{change_handle_kind_color, change_handle_kind_label, create_handle_kind, delete_handle_kind, rename_handle_kind};
