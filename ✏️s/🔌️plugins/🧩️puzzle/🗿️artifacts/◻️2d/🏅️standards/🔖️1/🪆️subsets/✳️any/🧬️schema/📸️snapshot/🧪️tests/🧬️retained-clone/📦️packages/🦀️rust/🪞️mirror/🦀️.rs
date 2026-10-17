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
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod retained_clone_tests;
                    }
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "."]
                        pub mod connect_handles_preparation {
                            #[path = "../../../../../../🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🪢️edge/🦀️.rs"]
                            pub mod edge;
                        }
                        #[path = "../../../../../../🧬️mutations/🏷️literal-id-inverse/🦀️.rs"]
                        pub mod literal_id_inverse;
                        #[path = "../../../../../../🧬️mutations/🟢️node/🌱create-node/🎮️prepare/🦀️.rs"]
                        pub mod create_node_preparation;
                        #[path = "../../../../../../🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/🦀️.rs"]
                        pub mod delete_node_preparation;
                        pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::preparation::native_preparation_child;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::selection;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "../../../../../../🧬️mutations/🟢️node/🌱create-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧬️mutations/🟢️node/🌱create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧬️mutations/🟢️node/🌱create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "../../../../../../🧬️mutations/🟢️node/🗑️delete-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧬️mutations/🟢️node/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧬️mutations/🟢️node/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
                        pub mod add_node_handle {
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➕add-node-handle/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➕add-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➕add-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_node_handle {
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➖remove-node-handle/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➖remove-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧬️mutations/🔌️handle/➖remove-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
                        pub mod replace_edge_geometry {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_kind {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_tips {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::inverse;
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
                        pub mod change_manifest_id {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::inverse;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::inverse;
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
                        pub mod edit_target_region_label {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::inverse;
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
                    pub mod binary {
                        #[path = "../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
