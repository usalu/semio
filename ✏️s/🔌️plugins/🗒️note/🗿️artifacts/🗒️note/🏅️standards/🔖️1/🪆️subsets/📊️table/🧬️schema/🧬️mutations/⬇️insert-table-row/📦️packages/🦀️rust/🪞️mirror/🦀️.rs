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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../✳️any/🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../✳️any/🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../../../✳️any/🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "../../../../../../../✳️any/📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod table {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod insert_table_row {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_table_row {
                            #[path = "../../../../⬆️remove-table-row/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../⬆️remove-table-row/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../⬆️remove-table-row/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_table_column {
                            #[path = "../../../../➡️insert-table-column/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../➡️insert-table-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../➡️insert-table-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_table_column {
                            #[path = "../../../../⬅️remove-table-column/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../⬅️remove-table-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../⬅️remove-table-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "../../../../../../../🧱️block/🚪️io/🦀️.rs"]
                pub mod io;
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
            #[path = "."]
            pub mod io {
                #[path = "../../../../../../../✳️any/✏️editor/👥️presence/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../../../✳️any/✏️editor/👥️presence/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
        }
    }
}
