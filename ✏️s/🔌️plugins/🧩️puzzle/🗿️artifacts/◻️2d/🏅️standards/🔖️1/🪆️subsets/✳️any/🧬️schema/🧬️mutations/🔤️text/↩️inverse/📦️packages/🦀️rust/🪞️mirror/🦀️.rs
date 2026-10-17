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
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "."]
                        pub mod connect_handles_preparation {
                            #[path = "."]
                            pub mod edge {
                                pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::edge::*;
                                #[cfg(test)]
                                #[path = "../../../../../🔌️handle/🪢️connect-handles/🎮️prepare/🪢️edge/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod flag_preparation {
                            pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::flag_preparation::*;
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::flag_preparation::candidate::*;
                                #[cfg(test)]
                                #[path = "../../../../../🚩️flag/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path="../../../../../🚩️flag/🎮️prepare/🧪️tests/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod inverse {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::flag_preparation::inverse::*;
                                #[cfg(test)]
                                #[path = "../../../../../🚩️flag/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod text_preparation {
                            pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::text_preparation::*;
                            #[path = "."]
                            pub mod messages {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::text_preparation::messages::*;
                                #[cfg(test)]
                                #[path = "../../../../🎮️prepare/📨️messages/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path="../../../../🎮️prepare/🧪️tests/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod candidate {
                                pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::text_preparation::candidate::*;
                                #[cfg(test)]
                                #[path="../../../../🎮️prepare/📸️candidate/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "../../../🦀️.rs"]
                        pub mod text_inverse;
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
                        pub mod replace_node_geometry {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::inverse;
                        }
                        #[path = "."]
                        pub mod edit_node_text {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::inverse;
                        }
                        #[path = "."]
                        pub mod scale_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_visible {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_root {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_anchor {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::inverse;
                        }
                        #[path = "."]
                        pub mod remove_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::inverse;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::inverse;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::inverse;
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
                        #[path = "."]
                        pub mod create_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::inverse;
                        }
                        #[path = "."]
                        pub mod delete_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::inverse;
                        }
                        #[path = "."]
                        pub mod move_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::inverse;
                        }
                        #[path = "."]
                        pub mod resize_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_region_hidden {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_region_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_puzzle_2d_mut_any_10::standards::v1::subsets::any::io::binary::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_2d_mut_any_10::standards::v1::subsets::any::io::text::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::io::sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
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
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                                pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot;
        #[path="../../../../../../../✏️editor/📼️recorder/🦀️.rs"]
        pub mod recorder;
        #[path = "../../../../../../../✏️editor/🧰️kit/🦀️.rs"]
        pub mod kit;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
    }
}
pub use semio_s_artifact_puzzle_2d_mut_any_10::host;
