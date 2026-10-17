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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_note_note_kinds_catalog::standards::v1::subsets::any::io::binary::*;
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::svg;
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf;
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::png;
                                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::dxf;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "../../../🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️base/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::io::export::serializers::artifacts::dxf;
                            }
                        }
                    }
                }
                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::examples;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_note_note_diff::editor::note::support;
        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🎼️command-enum/🦀️.rs"]
        pub mod command_enum;
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"]
        pub mod retained;
        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::*;
        }
        pub use semio_s_artifact_note_note_diff::editor::note::presence;
        pub use semio_s_artifact_note_note_diff::editor::note::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::add_block;
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::delete_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::delete_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_input;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_submit;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::ink_apply_events;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::move_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::navigator_engagement_input;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_down;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_down_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_left;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_left_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_right;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_right_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_up;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_up_fast;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::patch_blocks;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::save_download;
            #[path = "."]
            pub mod set_active_example {
                pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_active_example::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera_zoom;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_eraser_radius;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::load_document_json;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_opacity;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_spacing;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_subdivisions;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_visible;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_pencil_width;
            pub use semio_s_artifact_note_note_mut_family_4::editor::note::commands::set_snap_enabled;
            #[path = "../../../🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-snap-grid-spacing/🦀️.rs"]
            pub mod set_snap_grid_spacing;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::*;
                    }
                    #[path = "."]
                    pub mod navigator {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::navigator::*;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::catalogue;
            pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::document;
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs"]
                pub mod semantic_contract;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod note {
        #[path = "../../../🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_note_note_mut_family::viewer::note::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_note_note_diff::viewer::note::modes::view::windows::*;
                    #[path = "."]
                    pub mod composite {
                        pub use semio_s_artifact_note_note_diff::viewer::note::modes::view::windows::composite::*;
                        #[cfg(test)]
                        #[path = "../../../🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
