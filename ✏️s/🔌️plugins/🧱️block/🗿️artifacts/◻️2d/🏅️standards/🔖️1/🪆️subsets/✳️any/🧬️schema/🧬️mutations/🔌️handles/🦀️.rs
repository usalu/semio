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
#[path = "../🎨️change-handle-kind-color/🦀️.rs"]
pub mod change_handle_kind_color;
#[path = "../🔌️change-handle-kind-default-wire/🦀️.rs"]
pub mod change_handle_kind_default_wire_kind;
#[path = "../🔖️change-handle-kind-label/🦀️.rs"]
pub mod change_handle_kind_label;
#[path = "../🌿️create-handle/🦀️.rs"]
pub mod create_handle;
#[path = "../🌱️create-handle-kind/🦀️.rs"]
pub mod create_handle_kind;
#[path = "../❌️delete-handle/🦀️.rs"]
pub mod delete_handle;
#[path = "../🗑️delete-handle-kind/🦀️.rs"]
pub mod delete_handle_kind;
#[path = "../📍️move-handle/🦀️.rs"]
pub mod move_handle;
#[path = "../✒️rename-handle-kind/🦀️.rs"]
pub mod rename_handle_kind;
//#endregion 🪢️Mounts
