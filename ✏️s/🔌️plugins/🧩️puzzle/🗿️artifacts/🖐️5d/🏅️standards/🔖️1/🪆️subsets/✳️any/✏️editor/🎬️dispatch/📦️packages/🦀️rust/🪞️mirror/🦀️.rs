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
                        pub mod change_part_2d_hidden {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_locked {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::inverse;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::inverse;
                        }
                        #[path = "."]
                        pub mod rename_puzzle5d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::inverse;
                        }
                        #[path = "."]
                        pub mod change_domain {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::inverse;
                        }
                        #[path = "."]
                        pub mod change_description {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::inverse;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::inverse;
                        }
                        #[path = "."]
                        pub mod create_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod delete_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_hidden {
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::*;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::diff;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_locked {
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::*;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::diff;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::inverse;
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
            #[path = "../../../../🧰️kit/➕️add-node-and-brush-part/🦀️.rs"]
            pub mod add_node_and_brush_part;
            #[path = "../../../../🧰️kit/🧱️board-events-and-fasteners/🦀️.rs"]
            pub mod board_events_and_fasteners;
            #[path = "../../../../🧰️kit/📚️example-and-brush-mesh/🦀️.rs"]
            pub mod example_and_brush_mesh;
        }
        #[path = "."]
        pub mod gestures {
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::gestures::proximity_connect;
        }
        pub use semio_s_artifact_puzzle_5d_mut_any_5::editor::puzzle5d::snapshot;
        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
        #[path = "../../../🦀️.rs"]
        pub mod dispatch;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
        }
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window;
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::precompute::*;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::precompute::brush;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::add_brush_part;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::apply_sun;
            #[path = "../../../../🎮️commands/🔁️cycle-brush-candidate/🦀️.rs"]
            pub mod cycle_brush_candidate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::open_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::close_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::hover_suggestion;
            #[path = "../../../../🎮️commands/✅️accept-suggestion/🦀️.rs"]
            pub mod accept_suggestion;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::delete_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::duplicate_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::engagement_abort;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::engagement_control_select;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::engagement_input;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::engagement_repeat_last;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::engagement_submit;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::register_brush_mesh;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::rotate_selection;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::scale_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::select_same_kind;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_brush_placement_contact_tolerance;
            #[path = "../../../../🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_camera_2d;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::open_import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::open_add_part_dialog;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grid_factor;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grid_spacing;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grid_visible;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grip_direction;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_grip_show;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_lod_automatic;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_lod_depth_variable;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_lod_manual;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_projection;
            pub use semio_s_artifact_puzzle_5d_mut_any_7::editor::puzzle5d::commands::set_selectable_kind;
            pub use semio_s_artifact_puzzle_5d_mut_any_7::editor::puzzle5d::commands::set_transform_gumball_flag;
            #[path = "../../../../🎮️commands/⚖️set-kind-weight/🦀️.rs"]
            pub mod set_kind_weight;
            #[path = "../../../../🎮️commands/🔭️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "../../../../🎮️commands/📡️set-proximity-radius/🦀️.rs"]
            pub mod set_proximity_radius;
            #[path = "../../../../🎮️commands/🧱️set-chunk-size/🦀️.rs"]
            pub mod set_chunk_size;
            #[path = "../../../../🎮️commands/🚩️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "../../../../🎮️commands/🧭️set-suggestion-offset/🦀️.rs"]
            pub mod set_suggestion_offset;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::translate_selection;
            #[path = "../../../../🎮️commands/🌍️world-relocate/🦀️.rs"]
            pub mod world_relocate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::focus_selection;
            #[path = "../../../../🎮️commands/➕️add-target-volume/🦀️.rs"]
            pub mod add_target_volume;
            #[path = "../../../../🎮️commands/🪦️delete-target-volume/🦀️.rs"]
            pub mod delete_target_volume;
            #[path = "../../../../🎮️commands/🚚️relocate-target-volume/🦀️.rs"]
            pub mod relocate_target_volume;
            #[path = "../../../../🎮️commands/🚩️set-target-volume-flag/🦀️.rs"]
            pub mod set_target_volume_flag;
            #[path = "../../../../🎮️commands/📐️set-voxel-dims/🦀️.rs"]
            pub mod set_voxel_dims;
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "../../../../📌️panels/⚙️settings/🦀️.rs"]
            pub mod settings;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::*;
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::options;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::windows::*;
                    #[path = "."]
                    pub mod board2d {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::grid;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::lod;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::select;
                        }
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush;
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::utilities::select;
                        }
                    }
                    #[path = "."]
                    pub mod world3d {
                        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::windows::world3d::*;
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
