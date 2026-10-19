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
pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::play_snapshot;

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
        #[path = "../../../🦀️.rs"]
                mod component;
        pub use component::*;

        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
        pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::retained;
        pub use semio_s_artifact_puzzle_5d_mut_any_6::editor::puzzle5d::gestures;
        pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit;
        pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::dispatch;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::config::schema;
        }
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::presence::*;
        }
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::precompute::*;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::precompute::fill;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::precompute::brush;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::apply_sun;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::cycle_brush_candidate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::open_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::close_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::hover_suggestion;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::accept_suggestion;
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
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_camera;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_camera_2d;
            pub use semio_s_artifact_puzzle_5d_store_preparation::editor::puzzle5d::commands::set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::commands::export_snapshot;
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
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_kind_weight;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_lod_mode;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_proximity_radius;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_chunk_size;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_selection_flag;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_suggestion_offset;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::translate_selection;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::world_relocate;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::add_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::delete_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::relocate_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_target_volume_flag;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_voxel_dims;
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::panels::catalogue;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::panels::artifact;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::panels::inspection;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::panels::settings;
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
                    pub mod board2d {
                        pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::modes::edit::windows::board2d::*;
                        #[path = "."]
                        pub mod actions {
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::actions::apply_board_events;
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::actions::set_camera;
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
                        pub mod actions {
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::world3d::actions::rotate_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::scale_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::set_camera;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::translate_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::world_relocate;
                        }
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::utilities::world_relocate;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::utilities::volume_brush;
                        }
                    }
                }
            }
        }
    }
}
