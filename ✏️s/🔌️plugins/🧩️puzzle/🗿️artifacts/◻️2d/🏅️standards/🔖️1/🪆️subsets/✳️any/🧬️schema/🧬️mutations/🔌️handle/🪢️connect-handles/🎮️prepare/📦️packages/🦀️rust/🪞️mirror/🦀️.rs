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
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::literal_id_inverse;
                        #[path = "."]
                        pub mod create_node_preparation {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::create_node_preparation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🟢️node/🌱create-node/🎮️prepare/🧪️tests/🦀️.rs"]
                            pub mod tests;
                            #[path = "../../../../../../🟢️node/🌱create-node/🎮️prepare/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::create_node_preparation::candidate::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🟢️node/🌱create-node/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod delete_node_preparation {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::delete_node_preparation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🟢️node/🗑️delete-node/🎮️prepare/🧪️tests/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod inverse {
                                pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::delete_node_preparation::inverse::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🟢️node/🗑️delete-node/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::delete_node_preparation::candidate::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🟢️node/🗑️delete-node/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod move_node_preparation {
                            pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::move_node_preparation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧭️pose/📍move-node/🎮️prepare/🧪️tests/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::move_node_preparation::candidate::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧭️pose/📍move-node/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod inverse {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::move_node_preparation::inverse::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧭️pose/📍move-node/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "../../../🦀️.rs"]
                        pub mod connect_handles_preparation;
                        pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::text_preparation;
                        #[path = "."]
                        pub mod text_inverse {
                            pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::schema::preparation::text_inverse::*;
                            #[cfg(test)]
                            #[path="../../../../../../🔤️text/↩️inverse/🧪️tests/🦀️.rs"]
                            pub mod tests;
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
                        pub mod delete_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::inverse;
                        }
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::inverse;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_visible {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::text::snapshot;
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
        pub mod recorder {
            pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder::*;
            #[cfg(test)]
            #[path = "../../../../../../../../✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
            #[cfg(test)]
            #[path = "../../../../../../../../✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod brush;
        }
    }
}
