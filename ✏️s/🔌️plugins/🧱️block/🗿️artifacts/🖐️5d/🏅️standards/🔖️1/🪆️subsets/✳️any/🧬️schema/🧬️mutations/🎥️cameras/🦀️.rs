//! 🎥️ Block 5d cameras mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎥move-camera2d/🦀️.rs"]
pub mod move_camera2d;
#[path = "../🎬move-camera3d/🦀️.rs"]
pub mod move_camera3d;
#[path = "../🔍scale-camera2d/🦀️.rs"]
pub mod scale_camera2d;
#[path = "../🔎scale-camera3d/🦀️.rs"]
pub mod scale_camera3d;
//#endregion 🪢️Mounts
