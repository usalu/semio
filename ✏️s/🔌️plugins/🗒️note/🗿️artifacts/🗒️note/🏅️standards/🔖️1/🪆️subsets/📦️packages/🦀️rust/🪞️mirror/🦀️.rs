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
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️diff-apply/🦀️.rs"]
                        pub mod diff_apply_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod diff {
                            pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::io::text::diff::*;
                            #[cfg(test)]
                            #[path = "../../../✳️any/🚪️io/📝️text/🔺️diff/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod ink {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_pencil_width {
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_eraser_radius {
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_block_ink_width {
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_block_ink_stroke {
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod text {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_text {
                            #[path = "../../../📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod math {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_math {
                            #[path = "../../../🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod io {
                    #[path = "../../../🧱️block/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_block_locked {
                            #[path = "../../../🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/↩️inverse/🦀️.rs"]
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
        #[path = "."]
        pub mod presence {
            #[path = "../../../✳️any/✏️editor/👥️presence/🚪️io/🦀️.rs"]
            pub mod io;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod note {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_note_note_diff::viewer::note::modes::view::windows;
            }
        }
    }
}
