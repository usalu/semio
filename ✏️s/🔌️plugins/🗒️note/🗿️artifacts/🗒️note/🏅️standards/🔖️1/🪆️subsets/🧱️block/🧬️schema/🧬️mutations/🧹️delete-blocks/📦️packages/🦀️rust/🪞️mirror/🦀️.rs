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
                    pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff;
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::inverse;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::inverse;
                        }
                        #[path = "."]
                        pub mod delete_blocks {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod duplicate_block {
                            #[path = "../../../../📋️duplicate-block/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📋️duplicate-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📋️duplicate-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod duplicate_blocks {
                            #[path = "../../../../👥️duplicate-blocks/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../👥️duplicate-blocks/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../👥️duplicate-blocks/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
                            #[path = "../../../../../../../📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../../📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../../📜️document/🧬️schema/🧬️mutations/🏷️rename-note/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
        pub use semio_s_artifact_note_note_diff::editor::note::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🎥️camera/🦀️.rs"]
                            pub mod camera;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧹️eraser-point/🦀️.rs"]
                            pub mod eraser_point;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser-stroke/🦀️.rs"]
                            pub mod eraser_stroke;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🔲️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/✏️pencil/🦀️.rs"]
                            pub mod pencil;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧲️snap/🦀️.rs"]
                            pub mod snap;
                        }
                    }
                    #[path = "."]
                    pub mod navigator {
                        #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔲️grid-visible/🦀️.rs"]
                            pub mod grid_visible;
                            #[path = "../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔍️zoom/🦀️.rs"]
                            pub mod zoom;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../../../✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../../../../✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../../../../../✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
