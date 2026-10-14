//! 🎭️ block-3d-editor modes.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::ActionDescriptor;

/// 👁️✏️ Plain string tag (NOT the authoring trait's `APP_ID` — that const is removed, contract §2.1) —
/// stays a controller/action-factory id, exactly the way `world_3d_scene`'s `controller_id` needs a
/// stable string distinct from the derived `surface_app_id()`.
pub const BLOCK3D_PLAY_APP_ID: &str = "block3d-play";

pub const BLOCK3D_PLAY_SURFACE_ID: &str = "block3d.play.world";

pub const BLOCK3D_WORLD_OBJECT_ID: &str = "block3d-object";

pub const BLOCK3D_UTILITY_SURFACE_BRUSH: &str = "surfaceBrush";

/// 🪟️ Bridges window chrome (`☑️options/*`'s [`semio_framework_plugin::WindowMeasure`]s), which still
/// carries the retained WGPU action descriptor rather than a contract action binding.
pub fn block3d_window_action(action: &str, args: Option<semio_framework_value::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK3D_PLAY_APP_ID.into(), action: action.into(), args }
}

//#region 🪢️Mounts
#[path = "✏️edit/🦀️.rs"]
pub mod edit;
#[path = "../🌍️world/🦀️.rs"]
pub mod world;
//#endregion 🪢️Mounts
