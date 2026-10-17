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
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod move_part_2d {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_moves_part_a;
                        }
                        #[path = "."]
                        pub mod replace_part_2d_geometry {
                            #[cfg(test)]
                            #[path = "../../../../../../🧊replace-part2d-geometry/🧪️tests/🔳️circle/🦀️.rs"]
                            pub mod tests_circle_to_rectangle;
                        }
                        #[path = "."]
                        pub mod edit_part_2d_text {
                            #[cfg(test)]
                            #[path = "../../../../../../✏️edit-part2d-text/🧪️tests/✏️retitles-part-a/🦀️.rs"]
                            pub mod tests_retitles_part_a;
                        }
                        #[path = "."]
                        pub mod change_part_2d_icon {
                            #[cfg(test)]
                            #[path = "../../../../../../🎨change-part2d-icon/🧪️tests/🎨️swaps-icon/🦀️.rs"]
                            pub mod tests_swaps_icon;
                        }
                        #[path = "."]
                        pub mod change_part_2d_hidden {
                            #[cfg(test)]
                            #[path = "../../../../../../🙈change-part2d-hidden/🧪️tests/🙈️hides-part-a/🦀️.rs"]
                            pub mod tests_hides_part_a;
                        }
                        #[path = "."]
                        pub mod change_part_2d_locked {
                            #[cfg(test)]
                            #[path = "../../../../../../🔒change-part2d-locked/🧪️tests/🔒️locks-part-a/🦀️.rs"]
                            pub mod tests_locks_part_a;
                        }
                        #[path = "."]
                        pub mod move_part_3d {
                            #[cfg(test)]
                            #[path = "../../../../../../🚀move-part3d/🧪️tests/🗺️moves-part-a-in-world/🦀️.rs"]
                            pub mod tests_moves_part_a_in_world;
                        }
                        #[path = "."]
                        pub mod rotate_part_3d {
                            #[cfg(test)]
                            #[path = "../../../../../../🔃rotate-part3d/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                            pub mod tests_half_turn_about_z;
                        }
                        #[path = "."]
                        pub mod scale_part_3d {
                            #[cfg(test)]
                            #[path = "../../../../../../📏scale-part3d/🧪️tests/📏️uniform-double/🦀️.rs"]
                            pub mod tests_uniform_double;
                        }
                        #[path = "."]
                        pub mod change_part_3d_mesh {
                            #[cfg(test)]
                            #[path = "../../../../../../🧱change-part3d-mesh/🧪️tests/🕸️repoints-mesh/🦀️.rs"]
                            pub mod tests_repoints_mesh;
                        }
                        #[path = "."]
                        pub mod edit_part_3d_label {
                            #[cfg(test)]
                            #[path = "../../../../../../🖋️edit-part3d-label/🧪️tests/🔤️relabels-part-a/🦀️.rs"]
                            pub mod tests_relabels_part_a;
                        }
                        #[path = "."]
                        pub mod change_part_kind {
                            #[cfg(test)]
                            #[path = "../../../../../../🏗️change-part-kind/🧪️tests/🏷️reassigns-kind/🦀️.rs"]
                            pub mod tests_reassigns_kind;
                        }
                        #[path = "."]
                        pub mod change_part_anchor {
                            #[cfg(test)]
                            #[path = "../../../../../../⚓change-part-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
                            pub mod tests_fixed_to_derived;
                        }
                        #[path = "."]
                        pub mod add_part_grip {
                            #[cfg(test)]
                            #[path = "../../../../../../➕add-part-grip/🧪️tests/✊️appends-grip-3/🦀️.rs"]
                            pub mod tests_appends_grip_3;
                        }
                        #[path = "."]
                        pub mod remove_part_grip {
                            #[cfg(test)]
                            #[path = "../../../../../../➖remove-part-grip/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_grip_1_and_severs_fastener;
                            #[cfg(test)]
                            #[path = "../../../../../../➖remove-part-grip/🧪️tests/➖removes-middle-grip/🦀️.rs"]
                            pub mod tests_removes_the_middle_grip_and_its_fastener;
                        }
                        #[path = "."]
                        pub mod replace_part_grip {
                            #[cfg(test)]
                            #[path = "../../../../../../🔌replace-part-grip/🧪️tests/🏷️rekinds-grip-1/🦀️.rs"]
                            pub mod tests_rekinds_grip_1;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            #[cfg(test)]
                            #[path = "../../../../../../🪢️connect-grips/🧪️tests/🔩️adds-second-fastener/🦀️.rs"]
                            pub mod tests_adds_second_fastener;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            #[cfg(test)]
                            #[path = "../../../../../../✂️disconnect-grips/🧪️tests/🚫️removes-fast-1/🦀️.rs"]
                            pub mod tests_removes_fast_1;
                            #[cfg(test)]
                            #[path = "../../../../../../✂️disconnect-grips/🧪️tests/✂️severs-middle-fastener/🦀️.rs"]
                            pub mod tests_severs_the_middle_fastener;
                        }
                        #[path = "."]
                        pub mod replace_fastener_geometry {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧮replace-fastener-geometry/🧪️tests/📍️repositions/🦀️.rs"]
                            pub mod tests_repositions_fast_1;
                        }
                        #[path = "."]
                        pub mod change_fastener_kind {
                            #[cfg(test)]
                            #[path = "../../../../../../🎯change-fastener-kind/🧪️tests/🏷️rekinds-fast-1/🦀️.rs"]
                            pub mod tests_rekinds_fast_1;
                        }
                        #[path = "."]
                        pub mod rename_puzzle5d {
                            #[cfg(test)]
                            #[path = "../../../../../../🏷️rename-puzzle5d/🧪️tests/📃️relabels-document/🦀️.rs"]
                            pub mod tests_relabels_document;
                        }
                        #[path = "."]
                        pub mod change_domain {
                            #[cfg(test)]
                            #[path = "../../../../../../🌐change-domain/🧪️tests/⚙️architecture/🦀️.rs"]
                            pub mod tests_architecture_to_engineering;
                        }
                        #[path = "."]
                        pub mod change_description {
                            #[cfg(test)]
                            #[path = "../../../../../../🗒️change-description/🧪️tests/📝️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_description;
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
        #[path = "../../../../../../../../✏️editor/👆️gestures/🦀️.rs"]
        pub mod gestures;
    }
}
