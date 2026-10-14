//! 🎛️ Puzzle3d runtime commands — the action arms that drive the editing session without editing the document: view and
//! projection options, vortex display, the precompute weights and settings, brush suggestions and the engagement HUD.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "☀️apply-sun/🦀️.rs"]
pub mod apply_sun;
#[path = "🔒️close-vortex-suggestions/🦀️.rs"]
pub mod close_vortex_suggestions;
#[path = "🔁️cycle-candidate/🦀️.rs"]
pub mod cycle_candidate;
#[path = "🛑️engagement-abort/🦀️.rs"]
pub mod engagement_abort;
#[path = "🎛️engagement-control-select/🦀️.rs"]
pub mod engagement_control_select;
#[path = "⌨️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "🔂️engagement-repeat-last/🦀️.rs"]
pub mod engagement_repeat_last;
#[path = "📨️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "🖱️hover-suggestion/🦀️.rs"]
pub mod hover_suggestion;
#[path = "🔓️open-vortex-suggestions/🦀️.rs"]
pub mod open_vortex_suggestions;
#[path = "📋️register-brush-mesh/🦀️.rs"]
pub mod register_brush_mesh;
#[path = "🤖️set-automatic/🦀️.rs"]
pub mod set_automatic;
#[path = "🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
pub mod set_brush_placement_contact_tolerance;
#[path = "📷️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "🧩️set-chunk-size/🦀️.rs"]
pub mod set_chunk_size;
#[path = "📉️set-depth-variable/🦀️.rs"]
pub mod set_depth_variable;
#[path = "🧮️set-fill-count/🦀️.rs"]
pub mod set_fill_count;
#[path = "⚖️set-kind-weight/🦀️.rs"]
pub mod set_kind_weight;
#[path = "✋️set-manual/🦀️.rs"]
pub mod set_manual;
#[path = "📽️set-projection/🦀️.rs"]
pub mod set_projection;
#[path = "📡️set-proximity-radius/🦀️.rs"]
pub mod set_proximity_radius;
#[path = "☑️set-selectable-kind/🦀️.rs"]
pub mod set_selectable_kind;
#[path = "🧲️set-snap-enabled/🦀️.rs"]
pub mod set_snap_enabled;
#[path = "↔️set-spacing/🦀️.rs"]
pub mod set_spacing;
#[path = "🕹️set-transform-gumball-flag/🦀️.rs"]
pub mod set_transform_gumball_flag;
#[path = "🧭️set-vortex-direction/🦀️.rs"]
pub mod set_vortex_direction;
#[path = "🌀️set-vortex-show/🦀️.rs"]
pub mod set_vortex_show;
#[path = "📐️set-voxel-dims/🦀️.rs"]
pub mod set_voxel_dims;
#[path = "🎣️target-brush-suggestions/🦀️.rs"]
pub mod target_brush_suggestions;
//#endregion 🌳️Modules
