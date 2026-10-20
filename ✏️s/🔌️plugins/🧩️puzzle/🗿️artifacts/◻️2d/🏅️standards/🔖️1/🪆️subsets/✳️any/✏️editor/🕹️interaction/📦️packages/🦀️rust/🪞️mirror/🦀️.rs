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
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod drag_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::text::mutations;
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
        #[path = "../../../🦀️.rs"]
        pub mod interaction;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
        }
        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../🎮️commands/🧊️document/🌱️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../../../🎮️commands/🎛️runtime/🔒️close-handle-suggestions/🦀️.rs"]
            pub mod close_handle_suggestions;
            #[path = "../../../../🎮️commands/🎛️runtime/🎣️target-brush-suggestions/🦀️.rs"]
            pub mod target_brush_suggestions;
            #[path = "../../../../🎮️commands/🎛️runtime/🔁️cycle-candidate/🦀️.rs"]
            pub mod cycle_candidate;
            #[path = "../../../../🎮️commands/🧊️document/💞️create-edge/🦀️.rs"]
            pub mod create_edge;
            #[path = "../../../../🎮️commands/🧊️document/💔️delete-edge/🦀️.rs"]
            pub mod delete_edge;
            pub mod proximity_connect {
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::commands::proximity_connect::*;
                pub use crate::editor::puzzle2d::interaction::proximity_connect;
            }
            #[path = "../../../../🎮️commands/🎛️runtime/📡️set-proximity-radius/🦀️.rs"]
            pub mod set_proximity_radius;
            #[path = "../../../../🎮️commands/🧊️document/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../../🎮️commands/🧊️document/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "../../../../🎮️commands/🎛️runtime/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "../../../../🎮️commands/🎛️runtime/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "../../../../🎮️commands/🎛️runtime/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../../../🎮️commands/🧊️document/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "../../../../🎮️commands/🧊️document/⚛️force-layout/🦀️.rs"]
            pub mod force_layout;
            #[path = "../../../../🎮️commands/🎛️runtime/📊️lod-scale-json/🦀️.rs"]
            pub mod lod_scale_json;
            #[path = "../../../../🎮️commands/🎛️runtime/🔓️open-handle-suggestions/🦀️.rs"]
            pub mod open_handle_suggestions;
            #[path = "../../../../🎮️commands/🎛️runtime/🖱️hover-suggestion/🦀️.rs"]
            pub mod hover_suggestion;
            #[path = "../../../../🎮️commands/🧊️document/🩹️patch-inspector/🦀️.rs"]
            pub mod patch_inspector;
            #[path = "../../../../🎮️commands/🧊️document/🧬️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "../../../../🎮️commands/🎛️runtime/⚖️set-brush-kind-weights/🦀️.rs"]
            pub mod set_brush_kind_weights;
            #[path = "../../../../🎮️commands/🎛️runtime/📏️set-brush-node-size/🦀️.rs"]
            pub mod set_brush_node_size;
            #[path = "../../../../🎮️commands/🎛️runtime/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../../../🎮️commands/🎛️runtime/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_2d_graph_manifest::editor::puzzle2d::modes::edit::windows::overview::utilities::brush;
                            #[path = "."]
                            pub mod select {
                                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::select::*;
                                #[cfg(test)]
                                #[path = "../../../../🧪️tests/🧪️select-tool/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
