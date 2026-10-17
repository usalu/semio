//! 🪜️ Puzzle3d runtime commands — the action arms that drive the editing session without editing the document: view and (part 2).
//! projection options, vortex display, the precompute weights and settings, brush suggestions and the engagement HUD.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "../🛑️engagement-abort/🦀️.rs"]
pub mod engagement_abort;
#[path = "../⌨️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "../🔂️engagement-repeat-last/🦀️.rs"]
pub mod engagement_repeat_last;
#[path = "../📨️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "../🔓️open-vortex-suggestions/🦀️.rs"]
pub mod open_vortex_suggestions;
#[path = "../🤖️set-automatic/🦀️.rs"]
pub mod set_automatic;
#[path = "../📷️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "../📉️set-depth-variable/🦀️.rs"]
pub mod set_depth_variable;
#[path = "../🧮️set-fill-count/🦀️.rs"]
pub mod set_fill_count;
#[path = "../✋️set-manual/🦀️.rs"]
pub mod set_manual;
#[path = "../📡️set-proximity-radius/🦀️.rs"]
pub mod set_proximity_radius;
#[path = "../🧲️set-snap-enabled/🦀️.rs"]
pub mod set_snap_enabled;
#[path = "../🕹️set-transform-gumball-flag/🦀️.rs"]
pub mod set_transform_gumball_flag;
#[path = "../🌀️set-vortex-show/🦀️.rs"]
pub mod set_vortex_show;
#[path = "../🎣️target-brush-suggestions/🦀️.rs"]
pub mod target_brush_suggestions;
//#endregion 🌳️Modules
