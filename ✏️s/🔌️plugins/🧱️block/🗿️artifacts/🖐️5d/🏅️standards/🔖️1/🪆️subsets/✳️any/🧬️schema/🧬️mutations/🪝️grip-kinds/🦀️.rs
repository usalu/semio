//! 🪝️ Block 5d grip kinds mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎨change-grip-kind-color/🦀️.rs"]
pub mod change_grip_kind_color;
#[path = "../🪢change-grip-kind-default-rope-kind/🦀️.rs"]
pub mod change_grip_kind_default_rope_kind;
#[path = "../🎫change-grip-kind-label/🦀️.rs"]
pub mod change_grip_kind_label;
#[path = "../🌱create-grip-kind/🦀️.rs"]
pub mod create_grip_kind;
#[path = "../❌delete-grip-kind/🦀️.rs"]
pub mod delete_grip_kind;
#[path = "../🖋️rename-grip-kind/🦀️.rs"]
pub mod rename_grip_kind;
//#endregion 🪢️Mounts
