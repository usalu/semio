//! 🧩️ Block 5d part kind mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📃️change-part-kind-description/🦀️.rs"]
pub mod change_part_kind_description;
#[path = "../🖼️change-part-kind-icon/🦀️.rs"]
pub mod change_part_kind_icon;
#[path = "../🏷️change-part-kind-label/🦀️.rs"]
pub mod change_part_kind_label;
#[path = "../📐change-part-kind-unit/🦀️.rs"]
pub mod change_part_kind_unit;
#[path = "../🔀️change-part-kind-variant/🦀️.rs"]
pub mod change_part_kind_variant;
#[path = "../✏️rename-part-kind/🦀️.rs"]
pub mod rename_part_kind;
#[path = "../🖌️update-part2d/🦀️.rs"]
pub mod update_part_2d;
#[path = "../🧊update-part3d/🦀️.rs"]
pub mod update_part_3d;
//#endregion 🪢️Mounts
