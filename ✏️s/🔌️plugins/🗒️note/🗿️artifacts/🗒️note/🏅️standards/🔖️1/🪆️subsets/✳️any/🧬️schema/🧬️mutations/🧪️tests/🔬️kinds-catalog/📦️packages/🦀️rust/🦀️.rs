//! 🔬️ kinds-catalog of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema;
pub use semio_s_artifact_note_note_schema::NoteSnapshot;
#[cfg(test)]
pub use crate::schema::diff::NoteDiff;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}
pub use semio_s_artifact_note_note_schema::{NoteBlockNode, NoteCamera};

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
                    pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod kinds_catalog;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
                        }
                    }
                    #[path = "../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod drag_blocks {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/🤏️nudges/🦀️.rs"]
                            pub mod tests_nudges_ink_and_the_whole_group_subtree;
                        }
                        #[path = "."]
                        pub mod rename_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🧪️tests/🏷️renames/🦀️.rs"]
                            pub mod tests_renames_the_table_block;
                        }
                        #[path = "."]
                        pub mod change_block_visible {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🧪️tests/🙈️hides/🦀️.rs"]
                            pub mod tests_hides_the_image_block;
                        }
                        #[path = "."]
                        pub mod change_block_locked {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            pub mod tests_locks_the_callout_group;
                        }
                        #[path = "."]
                        pub mod move_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/📍️move-block/🧪️tests/📍️repositions/🦀️.rs"]
                            pub mod tests_repositions_the_math_block;
                        }
                        #[path = "."]
                        pub mod resize_block {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🧪️tests/📐️enlarges/🦀️.rs"]
                            pub mod tests_enlarges_the_image_block;
                        }
                        #[path = "."]
                        pub mod change_block_font_size {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🧪️tests/🔤️enlarges/🦀️.rs"]
                            pub mod tests_enlarges_the_intro_font;
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
        #[path = "../../../../../../✏️editor/🎯️dispatch-ctx/🦀️.rs"]
        pub mod dispatch_ctx;
        #[path = "."]
        pub mod window {
            #[path = "../../../../../../✏️editor/🪟️window/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../../../✏️editor/🪟️window/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../✏️editor/🎮️commands/➕️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "../../../../../../✏️editor/🎮️commands/🗑️delete-block/🦀️.rs"]
            pub mod delete_block;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::*;
                    }
                }
            }
        }
    }
}
