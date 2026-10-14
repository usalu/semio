//! 🎮️ block-3d-editor commands.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🧱️add-representation/🦀️.rs"]
pub mod add_representation;
#[path = "🌀️add-vortex/🦀️.rs"]
pub mod add_vortex;
#[path = "🔘️add-vortex-kind/🦀️.rs"]
pub mod add_vortex_kind;
#[path = "🎨️edit/🦀️.rs"]
pub mod edit;
#[path = "🖌️hover-surface/🦀️.rs"]
pub mod hover_surface;
#[path = "👋️leave-surface/🦀️.rs"]
pub mod leave_surface;
#[path = "🏷️patch-object-kind/🦀️.rs"]
pub mod patch_object_kind;
#[path = "🩹️patch-representation/🦀️.rs"]
pub mod patch_representation;
#[path = "📍️place-vortex/🦀️.rs"]
pub mod place_vortex;
#[path = "🚮️remove-representation/🦀️.rs"]
pub mod remove_representation;
#[path = "➖️remove-vortex/🦀️.rs"]
pub mod remove_vortex;
#[path = "🗑️remove-vortex-kind/🦀️.rs"]
pub mod remove_vortex_kind;
#[path = "🎬️set-active-example/🦀️.rs"]
pub mod set_active_example;
#[path = "🪟️set-active-representation/🦀️.rs"]
pub mod set_active_representation;
#[path = "🔄️set-brush-flip/🦀️.rs"]
pub mod set_brush_flip;
#[path = "📏️set-brush-radius/🦀️.rs"]
pub mod set_brush_radius;
#[path = "🧷️set-brush-vortex-kind/🦀️.rs"]
pub mod set_brush_vortex_kind;
#[path = "🎥️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "↔️set-window-arrangement/🦀️.rs"]
pub mod set_window_arrangement;
#[path = "🖼️set-window-representations/🦀️.rs"]
pub mod set_window_representations;
#[path = "📐️set-window-spacing/🦀️.rs"]
pub mod set_window_spacing;
#[path = "🔀️toggle-window-representation/🦀️.rs"]
pub mod toggle_window_representation;
//#endregion 🪢️Mounts
