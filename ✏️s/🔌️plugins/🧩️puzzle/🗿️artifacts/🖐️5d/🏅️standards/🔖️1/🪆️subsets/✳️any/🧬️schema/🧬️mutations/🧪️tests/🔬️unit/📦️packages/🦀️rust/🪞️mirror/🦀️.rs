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

                    pub use semio_s_artifact_puzzle_5d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                        #[cfg(test)]
                        #[path = "../../../../🧪️selection-time-travel/🦀️.rs"]
                        pub mod selection_time_travel;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::puzzle5d_json;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_puzzle_5d_mut_any_8::standards::v1::subsets::any::io::binary::*;
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_puzzle_5d_mut_any_8::standards::v1::subsets::any::io::text;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
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
        pub mod session {
            #[path = "../../../../../../../✏️editor/🖌️session/🧱️support/🦀️.rs"]
            pub mod support;
        }

        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        #[path = "../../../../../../../✏️editor/🧭️view/🦀️.rs"]
        pub mod view;
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "../../../../../../../✏️editor/🪟️window/🦀️.rs"]
        pub mod window;
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../✏️editor/🎮️commands/📤️export-snapshot/🦀️.rs"]
            pub mod export_snapshot;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🌐️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs"]
                            pub mod lod;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🎯️select/🦀️.rs"]
                            pub mod select;
                        }
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                        }
                    }
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🌐️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🤏️grip/🦀️.rs"]
                            pub mod grip;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🔭️lod/🦀️.rs"]
                            pub mod lod;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🎥️projection/🦀️.rs"]
                            pub mod projection;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🎯️select/🦀️.rs"]
                            pub mod select;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/☀️sun/🦀️.rs"]
                            pub mod sun;
                        }
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🧊️volume-brush/🦀️.rs"]
                            pub mod volume_brush;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_mut_any_8::viewer::puzzle5d::*;
        #[cfg(test)]
        #[path = "../../../../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::windows::board2d;
                    pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::windows::world3d;
                }
            }
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
