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
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::lookup;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::testkit;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "."]
                        pub mod create_node_preparation {
                            #[path = "."]
                            pub mod inverse {
                                pub use semio_s_artifact_puzzle_2d_connect_handles_preparation::standards::v1::subsets::any::schema::preparation::create_node_preparation::inverse::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🟢️node/🌱create-node/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::literal_id_inverse;
                        #[path = "."]
                        pub mod connect_handles_preparation {
                            pub use semio_s_artifact_puzzle_2d_connect_handles_preparation::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod inverse {
                                pub use semio_s_artifact_puzzle_2d_mut_any_10::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::inverse::*;
                                #[cfg(test)]
                                #[path = "../../../../↩️inverse/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::edge;
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_connect_handles_preparation::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::candidate::*;
                                #[cfg(test)]
                                #[path = "../../../../📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::preparation::native_preparation_child;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::inverse;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::text::mutations;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🧊️document/🔗️proximity-connect/🦀️.rs"]
            pub mod proximity_connect;
        }
        #[path = "."]
        pub mod precompute {
            #[path = "../../../../../../../../../✏️editor/⏳️precompute/🪣️fill/🦀️.rs"]
            pub mod fill;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/🖼️canvas/🦀️.rs"]
                pub mod canvas;
                #[path = "."]
                pub mod options {
                    #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs"]
                    pub mod brush;
                    #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/☑️options/🔄️transform/🦀️.rs"]
                    pub mod transform;
                    #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs"]
                    pub mod lod;
                    #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs"]
                    pub mod select;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖍️area-brush/🦀️.rs"]
                            pub mod area_brush;
                            #[path = "../../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs"]
                            pub mod select;
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
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_mut_any_9::viewer::puzzle2d::*;
        #[cfg(test)]
        #[path = "../../../../../../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_puzzle_2d_mut_any::viewer::puzzle2d::modes::view::*;
                pub use semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows;
            }
        }
    }
}
