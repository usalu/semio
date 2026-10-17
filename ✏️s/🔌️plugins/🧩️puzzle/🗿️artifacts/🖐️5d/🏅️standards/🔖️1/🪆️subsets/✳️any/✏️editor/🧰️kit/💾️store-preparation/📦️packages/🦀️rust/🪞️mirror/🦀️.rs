//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                        }
                        #[path = "."]
                        pub mod delete_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::inverse;
                        }
                        #[path = "."]
                        pub mod remove_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::inverse;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
        #[path = "."]
        pub mod kit {
            #[path = "../../../🦀️.rs"]
            pub mod store_preparation;
        }
        pub use semio_s_artifact_puzzle_5d_mut_any_5::editor::puzzle5d::snapshot;
        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
        }
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../🎮️commands/🔓️open-vortex-suggestions/🦀️.rs"]
            pub mod open_vortex_suggestions;
            #[path = "../../../../../🎮️commands/🔒️close-vortex-suggestions/🦀️.rs"]
            pub mod close_vortex_suggestions;
            #[path = "../../../../../🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../../../🎮️commands/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "../../../../../🎮️commands/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "../../../../../🎮️commands/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "../../../../../🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../../../../🎮️commands/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            #[path = "../../../../../🎮️commands/📋️register-brush-mesh/🦀️.rs"]
            pub mod register_brush_mesh;
            #[path = "../../../../../🎮️commands/🗂️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "../../../../../🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
            pub mod set_brush_placement_contact_tolerance;
            #[path = "../../../../../🎮️commands/🖼️set-camera-2d/🦀️.rs"]
            pub mod set_camera_2d;
            #[path = "../../../../../🎮️commands/🎦️set-camera-3d/🦀️.rs"]
            pub mod set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            #[path = "../../../../../🎮️commands/📥️import-snapshot/🦀️.rs"]
            pub mod import_snapshot;
            #[path = "../../../../../🎮️commands/🗂️open-import-snapshot/🦀️.rs"]
            pub mod open_import_snapshot;
            #[path = "../../../../../🎮️commands/🗨️open-add-part-dialog/🦀️.rs"]
            pub mod open_add_part_dialog;
            #[path = "../../../../../🎮️commands/📐️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "../../../../../🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../../../../🎮️commands/↔️set-grid-spacing/🦀️.rs"]
            pub mod set_grid_spacing;
            #[path = "../../../../../🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../../../../🎮️commands/🧭️set-grip-direction/🦀️.rs"]
            pub mod set_grip_direction;
            #[path = "../../../../../🎮️commands/🤏️set-grip-show/🦀️.rs"]
            pub mod set_grip_show;
            #[path = "../../../../../🎮️commands/🤖️set-lod-automatic/🦀️.rs"]
            pub mod set_lod_automatic;
            #[path = "../../../../../🎮️commands/📉️set-lod-depth-variable/🦀️.rs"]
            pub mod set_lod_depth_variable;
            #[path = "../../../../../🎮️commands/✋️set-lod-manual/🦀️.rs"]
            pub mod set_lod_manual;
            #[path = "../../../../../🎮️commands/📽️set-projection/🦀️.rs"]
            pub mod set_projection;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::*;
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform;
                        }
                    }
                }
            }
        }
    }
}
