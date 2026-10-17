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
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_node_geometry {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_rejects_reshaping_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../🔳️squares/🦀️.rs"]
                            pub mod tests_squares_the_concrete_forest_seed;
                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            #[cfg(test)]
                            #[path = "../../../../../../🏗️change-node-kind/🧪️tests/🏷️reassigns/🦀️.rs"]
                            pub mod tests_reassigns_node_a_kind;
                            #[cfg(test)]
                            #[path = "../../../../../../🏗️change-node-kind/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_rekinding_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../🏗️change-node-kind/🧪️tests/🏗️rekinds/🦀️.rs"]
                            pub mod tests_rekinds_a_capsule_j_as_a_capsule_l;
                        }
                        #[path = "."]
                        pub mod edit_node_text {
                            #[cfg(test)]
                            #[path = "../../../../../../✏️edit-node-text/🧪️tests/✏️recodes/🦀️.rs"]
                            pub mod tests_recodes_a_capsule_id_code;
                            #[cfg(test)]
                            #[path = "../../../../../../✏️edit-node-text/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_recoding_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../✏️edit-node-text/🧪️tests/✏️retitles-node-a/🦀️.rs"]
                            pub mod tests_retitles_node_a;
                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            #[cfg(test)]
                            #[path = "../../../../../../🎨change-node-icon/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_reiconing_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../🎨change-node-icon/🧪️tests/🎨️swaps-a-capsule-icon/🦀️.rs"]
                            pub mod tests_swaps_a_capsule_icon;
                            #[cfg(test)]
                            #[path = "../../../../../../🎨change-node-icon/🧪️tests/🎨️swaps-node-a-icon/🦀️.rs"]
                            pub mod tests_swaps_node_a_icon;
                        }
                        #[path = "."]
                        pub mod scale_node {
                            #[cfg(test)]
                            #[path = "../../../../../../../🧭️pose/📏scale-node/🧪️tests/🧱️zero-scale/🦀️.rs"]
                            pub mod tests_zero_scale;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧭️pose/📏scale-node/🧪️tests/📏️doubles-node-a/🦀️.rs"]
                            pub mod tests_doubles_node_a;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧭️pose/📏scale-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_scaling_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧭️pose/📏scale-node/🧪️tests/📏️scales/🦀️.rs"]
                            pub mod tests_scales_a_capsule_by_three_halves;
                        }
                        #[path = "."]
                        pub mod change_node_visible {
                            #[cfg(test)]
                            #[path = "../../../../../../👁️change-node-visible/🧪️tests/🙈️hides-a-capsule/🦀️.rs"]
                            pub mod tests_hides_a_capsule;
                            #[cfg(test)]
                            #[path = "../../../../../../👁️change-node-visible/🧪️tests/🙈️hides-node-a/🦀️.rs"]
                            pub mod tests_hides_node_a;
                            #[cfg(test)]
                            #[path = "../../../../../../👁️change-node-visible/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_hiding_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_node_locked {
                            #[cfg(test)]
                            #[path = "../../../../../../🔒change-node-locked/🧪️tests/🔒️locks-node-a/🦀️.rs"]
                            pub mod tests_locks_node_a;
                            #[cfg(test)]
                            #[path = "../../../../../../🔒change-node-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            pub mod tests_locks_the_first_storey_tambour;
                            #[cfg(test)]
                            #[path = "../../../../../../🔒change-node-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_locking_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_node_root {
                            #[cfg(test)]
                            #[path = "../../../../../../🌟change-node-root/🧪️tests/🌳️promotes-node-a-to-root/🦀️.rs"]
                            pub mod tests_promotes_node_a_to_root;
                            #[cfg(test)]
                            #[path = "../../../../../../🌟change-node-root/🧪️tests/🌳️promotes/🦀️.rs"]
                            pub mod tests_promotes_the_base_to_root;
                            #[cfg(test)]
                            #[path = "../../../../../../🌟change-node-root/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_rooting_a_capsule_the_board_never_held;
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
        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🧊️document/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "../../../../../../../../../✏️editor/🎮️commands/🧊️document/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
                        }
                    }
                }
            }
        }
    }
}
