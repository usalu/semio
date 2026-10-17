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
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod delete_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::inverse;
                        }
                        #[path = "."]
                        pub mod change_node_anchor {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_derives_a_capsule_pose_from_its_door_edge;
                            #[cfg(test)]
                            #[path = "../../../../⚓️fixed-to-derived/🦀️.rs"]
                            pub mod tests_fixed_to_derived;
                            #[cfg(test)]
                            #[path = "../../../../🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_anchoring_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod add_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::add_node_handle::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➕add-node-handle/🧪️tests/🧱️zero-radius-handle/🦀️.rs"]
                            pub mod tests_zero_radius_handle;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➕add-node-handle/🧪️tests/➕️adds/🦀️.rs"]
                            pub mod tests_adds_a_third_slot_door_to_the_tambour;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➕add-node-handle/🧪️tests/➕️appends-handle-3-to-node-b/🦀️.rs"]
                            pub mod tests_appends_handle_3_to_node_b;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➕add-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_adding_a_door_to_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod remove_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➖remove-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_removing_a_door_the_tambour_never_had;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➖remove-node-handle/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_a_tambour_door_and_severs_its_capsule_edge;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➖remove-node-handle/🧪️tests/🚫️removes2/🦀️.rs"]
                            pub mod tests_removes_handle_2_and_severs_edge;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/➖remove-node-handle/🧪️tests/➖removes-middle-handle/🦀️.rs"]
                            pub mod tests_removes_the_middle_handle_and_severs_its_edge;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🔌replace-node-handle/🧪️tests/🧱️negative-scale/🦀️.rs"]
                            pub mod tests_negative_scale;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🔌replace-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_replacing_a_door_the_tambour_never_had;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🔌replace-node-handle/🧪️tests/🔌️rekinds/🦀️.rs"]
                            pub mod tests_rekinds_an_unconnected_tambour_door;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🪢️connect-handles/↩️inverse/📑️ordered-restoration/🧪️tests/🦀️.rs"]
                            pub mod ordered_restoration_tests;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🪢️connect-handles/🧪️tests/🪢️adds-second-edge/🦀️.rs"]
                            pub mod tests_adds_second_edge;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🪢️connect-handles/🧪️tests/⏸️keeps/🦀️.rs"]
                            pub mod tests_keeps_an_edge_the_tower_already_holds;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/🪢️connect-handles/🧪️tests/🪢️rewires/🦀️.rs"]
                            pub mod tests_rewires_the_capsule_the_subgraph_left_loose;
                        }
                        #[path = "."]
                        pub mod disconnect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/✂️disconnect-handles/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_severing_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/✂️disconnect-handles/🧪️tests/🚫️removes-edge-1/🦀️.rs"]
                            pub mod tests_removes_edge_1;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/✂️disconnect-handles/🧪️tests/✂️severs-middle-edge/🦀️.rs"]
                            pub mod tests_severs_the_middle_edge;
                            #[cfg(test)]
                            #[path = "../../../../../../../🔌️handle/✂️disconnect-handles/🧪️tests/✂️severs/🦀️.rs"]
                            pub mod tests_severs_a_capsule_from_the_first_storey_tambour;
                        }
                        #[path = "."]
                        pub mod replace_edge_geometry {
                            #[cfg(test)]
                            #[path = "../../../../../../../〰️edge/🧮replace-edge-geometry/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_reposing_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../../〰️edge/🧮replace-edge-geometry/🧪️tests/🧮️reposes/🦀️.rs"]
                            pub mod tests_reposes_a_capsule_door_edge;
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
        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🎛️runtime/🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
            pub mod set_brush_placement_contact_tolerance;
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🎛️runtime/🫂️set-brush-placement-overlap-budget/🦀️.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🎛️runtime/🔭️set-lod-mode-for-pane/🦀️.rs"]
            pub mod set_lod_mode_for_pane;
        }
    }
}
