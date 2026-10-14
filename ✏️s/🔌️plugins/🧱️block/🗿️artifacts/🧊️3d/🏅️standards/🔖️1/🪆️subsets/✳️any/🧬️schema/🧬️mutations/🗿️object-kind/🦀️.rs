//! 🗿️ Block 3d object kind mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📃️change-object-kind/🦀️.rs"]
pub mod change_object_kind_description;
#[path = "../🖼️change-object-kind-icon/🦀️.rs"]
pub mod change_object_kind_icon;
#[path = "../🏷️change-object-kind-label/🦀️.rs"]
pub mod change_object_kind_label;
#[path = "../📐change-object-kind-unit/🦀️.rs"]
pub mod change_object_kind_unit;
#[path = "../🔀️change-object-kind-variant/🦀️.rs"]
pub mod change_object_kind_variant;
#[path = "../✏️rename-object-kind/🦀️.rs"]
pub mod rename_object_kind;
//#endregion 🪢️Mounts
