//! 🔷️ Block 2d node kind mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📃️change-node-kind-description/🦀️.rs"]
pub mod change_node_kind_description;
#[path = "../🖼️change-node-kind-icon/🦀️.rs"]
pub mod change_node_kind_icon;
#[path = "../🏷️change-node-kind-label/🦀️.rs"]
pub mod change_node_kind_label;
#[path = "../📐️change-node-kind-unit/🦀️.rs"]
pub mod change_node_kind_unit;
#[path = "../🔀️change-node-kind-variant/🦀️.rs"]
pub mod change_node_kind_variant;
#[path = "../✏️rename-node-kind/🦀️.rs"]
pub mod rename_node_kind;
#[path = "../🖌️update-presentation/🦀️.rs"]
pub mod update_presentation;
//#endregion 🪢️Mounts
