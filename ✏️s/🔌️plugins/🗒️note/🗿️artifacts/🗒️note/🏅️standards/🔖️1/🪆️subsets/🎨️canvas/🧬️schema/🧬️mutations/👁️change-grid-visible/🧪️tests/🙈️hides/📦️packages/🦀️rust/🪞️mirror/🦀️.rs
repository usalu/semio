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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::examples;
            }
            #[path = "."]
            pub mod canvas {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_grid_visible {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_hides_the_grid;
                        }
                        #[path = "."]
                        pub mod change_grid_spacing {
                            #[cfg(test)]
                            #[path = "../../../../../../📏️change-grid-spacing/🧪️tests/📏️widens/🦀️.rs"]
                            pub mod tests_widens_grid_spacing;
                        }
                        #[path = "."]
                        pub mod change_grid_subdivisions {
                            #[cfg(test)]
                            #[path = "../../../../../../🔢️change-grid-subdivisions/🧪️tests/🔢️doubles/🦀️.rs"]
                            pub mod tests_doubles_grid_subdivisions;
                        }
                        #[path = "."]
                        pub mod change_grid_opacity {
                            #[cfg(test)]
                            #[path = "../../../../../../🌫️change-grid-opacity/🧪️tests/🌫️raises/🦀️.rs"]
                            pub mod tests_raises_grid_opacity;
                        }
                        #[path = "."]
                        pub mod change_snap_enabled {
                            #[cfg(test)]
                            #[path = "../../../../../../🧲️change-snap-enabled/🧪️tests/🧲️enables-snap/🦀️.rs"]
                            pub mod tests_enables_snap;
                        }
                        #[path = "."]
                        pub mod change_snap_grid_spacing {
                            #[cfg(test)]
                            #[path = "../../../../../../📐️change-snap-grid-spacing/🧪️tests/📐️halves/🦀️.rs"]
                            pub mod tests_halves_snap_grid_spacing;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_note {
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🧪️tests/🏷️retitles/🦀️.rs"]
                            pub mod tests_retitles_the_document;
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
    pub mod note {
        pub use semio_s_artifact_note_note_diff::editor::note::support;
        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;
        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::*;
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::schema;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🚫️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/📋️duplicate-block/🦀️.rs"]
            pub mod duplicate_block;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🪞️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/💬️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs"]
            pub mod ink_apply_events;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🧭️navigator-engagement-input/🦀️.rs"]
            pub mod navigator_engagement_input;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🩹️patch-blocks/🦀️.rs"]
            pub mod patch_blocks;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/💾️save-download/🦀️.rs"]
            pub mod save_download;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🗃️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🔭️set-camera-zoom/🦀️.rs"]
            pub mod set_camera_zoom;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🧽️set-eraser-radius/🦀️.rs"]
            pub mod set_eraser_radius;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/📄️load-document-json/🦀️.rs"]
            pub mod load_document_json;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🔲️set-grid-opacity/🦀️.rs"]
            pub mod set_grid_opacity;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/📏️set-grid-spacing/🦀️.rs"]
            pub mod set_grid_spacing;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/🔢️set-grid-subdivisions/🦀️.rs"]
            pub mod set_grid_subdivisions;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../../../../../../../../✳️any/✏️editor/🎮️commands/✏️set-pencil-width/🦀️.rs"]
            pub mod set_pencil_width;
        }
    }
}
