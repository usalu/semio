//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::play_snapshot;

                    pub use semio_s_artifact_puzzle_5d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::kind_catalogs;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::io::binary::mutations;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::capsule_dream;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::concrete_forest;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::nakagin_capsule_tower;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_puzzle5d::editor::puzzle5d::*;

        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
        pub use semio_s_artifact_puzzle_5d_puzzle5d::editor::puzzle5d::session;
        pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::retained;
        pub use semio_s_artifact_puzzle_5d_mut_any_6::editor::puzzle5d::gestures;
        pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit;
        pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::dispatch;
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_puzzle_5d_testkit as testkit;
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_puzzle_5d_testkit::puzzle5d_app_manifest_for_tests;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
        }
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::presence::*;
        }
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::precompute::*;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::apply_sun;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::cycle_brush_candidate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::close_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::hover_suggestion;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::accept_suggestion;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::delete_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::duplicate_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_abort;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_control_select;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_input;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_repeat_last;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::engagement_submit;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::register_brush_mesh;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::rotate_selection;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::scale_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::select_same_kind;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_brush_placement_contact_tolerance;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_camera;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_camera_2d;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::commands::export_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_add_part_dialog;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_factor;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_spacing;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_visible;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grip_direction;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grip_show;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_automatic;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_depth_variable;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_manual;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_projection;
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
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::catalogue;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::artifact;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::inspection;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::settings;
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
                    }
                    #[path = "."]
                    pub mod world3d {
                        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::windows::world3d::*;
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::utilities::volume_brush;
                        }
                    }
                }
            }
        }
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🤖️agent-lane/🦀️.rs"]
        pub mod agent_lane_tests;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️target-volumes/🦀️.rs"]
        pub mod target_volume_tests;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🧵️retained-wiring/🦀️.rs"]
        pub mod retained_wiring_tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_mut_any_8::viewer::puzzle5d::*;
    }
}
pub use semio_s_artifact_puzzle_5d_tests::host;
