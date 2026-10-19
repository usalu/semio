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
                        pub mod delete_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::inverse;
                        }
                        #[path = "."]
                        pub mod move_part_2d {
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::*;
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::diff;
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_2d_text {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::inverse;
                        }
                        #[path = "."]
                        pub mod move_part_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_3d_mesh {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_3d_label {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_kind {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_anchor {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::inverse;
                        }
                        #[path = "."]
                        pub mod replace_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::inverse;
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
                        pub mod replace_fastener_geometry {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::inverse;
                        }
                        #[path = "."]
                        pub mod change_fastener_kind {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection_2d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::inverse;
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
        pub mod gestures {
            #[path = "../../../../👆️gestures/🎛️selection-and-weights/🦀️.rs"]
            pub mod selection_and_weights;
            #[path = "../../../../👆️gestures/🩹️patches/🦀️.rs"]
            pub mod patches;
            #[path = "../../../../👆️gestures/🧲️proximity-connect/🦀️.rs"]
            pub mod proximity_connect;
        }

        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        #[path = "../../../🦀️.rs"]
        pub mod interaction;
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
        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::precompute::*;
            #[path = "../../../../🧠️precompute/🪣️fill/🦀️.rs"]
            pub mod fill;
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::precompute::geometry;
            #[path = "../../../../🧠️precompute/🖌️brush/🦀️.rs"]
            pub mod brush;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../🎮️commands/🖌️add-brush-part/🦀️.rs"]
            pub mod add_brush_part;
            #[path = "../../../../🎮️commands/☀️apply-sun/🦀️.rs"]
            pub mod apply_sun;
            #[path = "../../../../🎮️commands/🎣️target-brush-suggestions/🦀️.rs"]
            pub mod target_brush_suggestions;
            #[path = "../../../../🎮️commands/🖱️hover-suggestion/🦀️.rs"]
            pub mod hover_suggestion;
            #[path = "../../../../🎮️commands/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "."]
            pub mod proximity_connect {
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::commands::proximity_connect::*;
                #[cfg(test)]
                #[path = "../../../../🎮️commands/📡️proximity-connect/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "../../../../🎮️commands/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "../../../../🎮️commands/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "../../../../🎮️commands/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
            #[path = "../../../../🎮️commands/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "../../../../🎮️commands/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::options;
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush;
                        }
                    }
                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::grid;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::grip;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::lod;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::projection;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::select;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::sun;
                        }
                        #[path = "."]
                        pub mod utilities {
                            #[path = "."]
                            pub mod transform {
                                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::*;
                                #[cfg(test)]
                                #[path = "../../../../🧪️tests/🧪️transform-tool/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::utilities::world_relocate;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::utilities::volume_brush;
                        }
                    }
                }
            }
        }
    }
}
