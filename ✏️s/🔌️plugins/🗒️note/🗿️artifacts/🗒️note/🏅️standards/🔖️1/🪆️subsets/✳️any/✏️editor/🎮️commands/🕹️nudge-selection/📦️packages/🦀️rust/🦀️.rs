//! 🕹️ nudge-selection of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;

pub use crate::standards::v1::subsets::any::schema;
pub use semio_s_artifact_note_note_schema::NoteSnapshot;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}

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
        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::ink_apply_events;
            #[path = "../../🦀️.rs"]
            pub mod nudge_selection;
            #[path = "../../../⬇️nudge-selection-down/🦀️.rs"]
            pub mod nudge_selection_down;
            #[path = "../../../⏬️nudge-selection-down-fast/🦀️.rs"]
            pub mod nudge_selection_down_fast;
            #[path = "../../../⬅️nudge-selection-left/🦀️.rs"]
            pub mod nudge_selection_left;
            #[path = "../../../⏪️nudge-selection-left-fast/🦀️.rs"]
            pub mod nudge_selection_left_fast;
            #[path = "../../../➡️nudge-selection-right/🦀️.rs"]
            pub mod nudge_selection_right;
            #[path = "../../../⏩️nudge-selection-right-fast/🦀️.rs"]
            pub mod nudge_selection_right_fast;
            #[path = "../../../⬆️nudge-selection-up/🦀️.rs"]
            pub mod nudge_selection_up;
            #[path = "../../../⏫️nudge-selection-up-fast/🦀️.rs"]
            pub mod nudge_selection_up_fast;
        }
    }
}
