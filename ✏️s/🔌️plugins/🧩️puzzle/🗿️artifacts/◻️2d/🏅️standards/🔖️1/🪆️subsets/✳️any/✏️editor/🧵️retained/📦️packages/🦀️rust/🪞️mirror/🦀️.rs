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
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::inverse;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::inverse;
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
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot;
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder;
        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
        #[path = "../../../🦀️.rs"]
        pub mod retained;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::add_node;
            #[path = "../../../../🎮️commands/🧊️document/🎲️apply-board-events/🦀️.rs"]
            pub mod apply_board_events;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::close_handle_suggestions;
            #[path = "../../../../🎮️commands/🧊️document/✅️accept-suggestion/🦀️.rs"]
            pub mod accept_suggestion;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::cycle_candidate;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::create_edge;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::delete_edge;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::proximity_connect;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_proximity_radius;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::delete_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::duplicate_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_abort;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_control_select;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_input;
            #[path = "../../../../🎮️commands/🎛️runtime/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::focus_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::force_layout;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::lod_scale_json;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::open_handle_suggestions;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::hover_suggestion;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::patch_inspector;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::select_same_kind;
            pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::commands::set_active_example;
            #[path = "."]
            pub mod set_brush_kind_weights {
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_brush_kind_weights::*;
                #[cfg(test)]
                #[path = "../../../../🎮️commands/🎛️runtime/⚖️set-brush-kind-weights/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_brush_node_size;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_camera;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_factor;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_visible;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_selectable_kind;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_brush_placement_contact_tolerance;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_brush_placement_overlap_budget;
            #[path = "../../../../🎮️commands/🎛️runtime/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_lod_mode_for_pane;
            pub use semio_s_artifact_puzzle_2d_mut_any_6::editor::puzzle2d::commands::translate_selection;
            pub use semio_s_artifact_puzzle_2d_mut_any_6::editor::puzzle2d::commands::rotate_selection;
            #[path = "../../../../🎮️commands/🧊️document/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            pub use semio_s_artifact_puzzle_2d_graph_manifest::editor::puzzle2d::commands::export_snapshot;
            #[path = "../../../../🎮️commands/🧊️document/📥️import-snapshot/🦀️.rs"]
            pub mod import_snapshot;
            #[path = "../../../../🎮️commands/🧊️document/🗂️open-import-snapshot/🦀️.rs"]
            pub mod open_import_snapshot;
            #[path = "../../../../🎮️commands/🧊️document/🔖️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "../../../../🎮️commands/🎛️runtime/🧭️set-suggestion-offset/🦀️.rs"]
            pub mod set_suggestion_offset;
            #[path = "../../../../🎮️commands/🎛️runtime/🕹️set-transform-gumball-flag/🦀️.rs"]
            pub mod set_transform_gumball_flag;
            #[path = "../../../../🎮️commands/🧊️document/➕️add-target-region/🦀️.rs"]
            pub mod add_target_region;
            #[path = "../../../../🎮️commands/🧊️document/🪦️delete-target-region/🦀️.rs"]
            pub mod delete_target_region;
            #[path = "../../../../🎮️commands/🧊️document/🚚️relocate-target-region/🦀️.rs"]
            pub mod relocate_target_region;
            #[path = "../../../../🎮️commands/🧊️document/🚩️set-target-region-flag/🦀️.rs"]
            pub mod set_target_region_flag;
            #[path = "../../../../🎮️commands/🎛️runtime/🖍️set-area-brush-size/🦀️.rs"]
            pub mod set_area_brush_size;
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
                #[path = "../../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::canvas;
                #[path = "."]
                pub mod options {
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::brush;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::grid;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::transform;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::lod;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::select;
                }
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_2d_graph_manifest::editor::puzzle2d::modes::edit::windows::overview::utilities::brush;
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::area_brush;
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
                        }
                    }
                    #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️.rs"]
                    pub mod detail;
                    #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️.rs"]
                    pub mod selection;
                }
            }
        }
    }
}
