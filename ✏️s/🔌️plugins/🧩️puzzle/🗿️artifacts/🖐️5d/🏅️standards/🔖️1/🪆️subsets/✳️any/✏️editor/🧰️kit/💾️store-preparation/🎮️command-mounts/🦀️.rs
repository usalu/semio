//! 🎮️ The grid, lod, camera, selection, engagement and import command mounts that prepare the puzzle 5d store, held by their own crate so the store-preparation crate stays inside the source gate.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/🔓️open-vortex-suggestions/🦀️.rs"]
            pub mod open_vortex_suggestions;
            #[path = "../../../🎮️commands/🔒️close-vortex-suggestions/🦀️.rs"]
            pub mod close_vortex_suggestions;
            #[path = "../../../🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../🎮️commands/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "../../../🎮️commands/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "../../../🎮️commands/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "../../../🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../../🎮️commands/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            #[path = "../../../🎮️commands/📋️register-brush-mesh/🦀️.rs"]
            pub mod register_brush_mesh;
            #[path = "../../../🎮️commands/🗂️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "../../../🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
            pub mod set_brush_placement_contact_tolerance;
            #[path = "../../../🎮️commands/🖼️set-camera-2d/🦀️.rs"]
            pub mod set_camera_2d;
            #[path = "../../../🎮️commands/🎦️set-camera-3d/🦀️.rs"]
            pub mod set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            #[path = "../../../🎮️commands/📥️import-snapshot/🦀️.rs"]
            pub mod import_snapshot;
            #[path = "../../../🎮️commands/🗂️open-import-snapshot/🦀️.rs"]
            pub mod open_import_snapshot;
            #[path = "../../../🎮️commands/🗨️open-add-part-dialog/🦀️.rs"]
            pub mod open_add_part_dialog;
            #[path = "../../../🎮️commands/📐️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "../../../🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../../🎮️commands/↔️set-grid-spacing/🦀️.rs"]
            pub mod set_grid_spacing;
            #[path = "../../../🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../../🎮️commands/🧭️set-grip-direction/🦀️.rs"]
            pub mod set_grip_direction;
            #[path = "../../../🎮️commands/🤏️set-grip-show/🦀️.rs"]
            pub mod set_grip_show;
            #[path = "../../../🎮️commands/🤖️set-lod-automatic/🦀️.rs"]
            pub mod set_lod_automatic;
            #[path = "../../../🎮️commands/📉️set-lod-depth-variable/🦀️.rs"]
            pub mod set_lod_depth_variable;
            #[path = "../../../🎮️commands/✋️set-lod-manual/🦀️.rs"]
            pub mod set_lod_manual;
            #[path = "../../../🎮️commands/📽️set-projection/🦀️.rs"]
            pub mod set_projection;
        }
    }
}
