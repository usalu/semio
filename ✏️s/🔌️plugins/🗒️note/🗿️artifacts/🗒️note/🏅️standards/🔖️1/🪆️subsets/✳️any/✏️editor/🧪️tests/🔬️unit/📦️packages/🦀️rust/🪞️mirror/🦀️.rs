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
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        pub use semio_s_artifact_note_note_v1::editor::note::*;
        #[cfg(test)]
        #[path = "../../../🦀️.rs"]
        pub mod unit_tests;
        pub use semio_s_artifact_note_note_diff::editor::note::support;
        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;
        pub use semio_s_artifact_note_note_v1::editor::note::command_enum;
        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::*;
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::schema;
        }
        pub use semio_s_artifact_note_note_diff::editor::note::presence;
        pub use semio_s_artifact_note_note_diff::editor::note::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_note_note_v1::editor::note::commands::*;
            #[path = "."]
            pub mod add_block {
                pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::add_block::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/➕️add-block/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::delete_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::delete_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_input;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_submit;
            #[path = "."]
            pub mod ink_apply_events {
                pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::ink_apply_events::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🖊️ink-apply-events/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
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
            #[path = "."]
            pub mod save_download {
                pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::save_download::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/💾️save-download/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_active_example;
            #[path = "."]
            pub mod set_camera {
                pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera_zoom;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_eraser_radius;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::load_document_json;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_opacity;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_spacing;
            #[path = "."]
            pub mod set_grid_subdivisions {
                pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_subdivisions::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🔢️set-grid-subdivisions/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_visible;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_pencil_width;
            pub use semio_s_artifact_note_note_mut_family_4::editor::note::commands::set_snap_enabled;
            pub use semio_s_artifact_note_note_v1::editor::note::commands::set_snap_grid_spacing;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::*;
                    #[path = "."]
                    pub mod composite {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🖼️composite/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod navigator {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::navigator::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🧭️navigator/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod catalogue {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::catalogue::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::document::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
pub use semio_s_artifact_note_note_mutations::host;
