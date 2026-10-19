//! projection options, vortex display, the precompute weights and settings, brush suggestions and the engagement HUD.
//! 🎛️ Puzzle3d runtime commands — the action arms that drive the editing session without editing the document: view and

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
#[path = "🎛️engagement-control-select/🦀️.rs"]
pub mod engagement_control_select;
#[path = "🖱️hover-suggestion/🦀️.rs"]
pub mod hover_suggestion;
#[path = "📋️register-brush-mesh/🦀️.rs"]
pub mod register_brush_mesh;
#[path = "🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
pub mod set_brush_placement_contact_tolerance;
#[path = "🧩️set-chunk-size/🦀️.rs"]
pub mod set_chunk_size;
#[path = "⚖️set-kind-weight/🦀️.rs"]
pub mod set_kind_weight;
#[path = "📽️set-projection/🦀️.rs"]
pub mod set_projection;
#[path = "☑️set-selectable-kind/🦀️.rs"]
pub mod set_selectable_kind;
#[path = "↔️set-spacing/🦀️.rs"]
pub mod set_spacing;
#[path = "🧭️set-vortex-direction/🦀️.rs"]
pub mod set_vortex_direction;
#[path = "📐️set-voxel-dims/🦀️.rs"]
pub mod set_voxel_dims;
//#endregion 🌳️Modules

pub use semio_s_artifact_puzzle_3d_commands_runtime_2::{engagement_abort, engagement_input, engagement_repeat_last, engagement_submit, open_vortex_suggestions, set_automatic, set_camera, set_depth_variable, set_fill_count, set_manual, set_proximity_radius, set_snap_enabled, set_transform_gumball_flag, set_vortex_show, target_brush_suggestions};
