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
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::kind_catalogs;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_locked {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_rejects_locking_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod drag_selection_2d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::*;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-selection2d/🧪️tests/✋️drags-two-parts/🦀️.rs"]
                            pub mod tests_drags_two_parts;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-selection2d/🧪️tests/⚠️skips/🦀️.rs"]
                            pub mod tests_skips_locked_volume_ghost;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-selection2d/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-selection2d/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
                            pub mod tests_keeps_a_zero_offset;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️drag-selection2d/🧪️tests/🧱️no-targets/🦀️.rs"]
                            pub mod tests_no_targets;
                        }
                        #[path = "."]
                        pub mod drag_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️drag-selection3d/🧪️tests/🚚️drags/🦀️.rs"]
                            pub mod tests_drags_part_and_volume;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️drag-selection3d/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️drag-selection3d/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️drag-selection3d/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
                            pub mod tests_keeps_a_zero_offset;
                            #[cfg(test)]
                            #[path = "../../../../../../🚚️drag-selection3d/🧪️tests/🧱️repeated-targets/🦀️.rs"]
                            pub mod tests_repeated_targets;
                        }
                        #[path = "."]
                        pub mod rotate_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-selection3d/🧪️tests/🔄️turns/🦀️.rs"]
                            pub mod tests_turns_part_and_volume;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-selection3d/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-selection3d/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-selection3d/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
                            pub mod tests_keeps_a_zero_angle;
                            #[cfg(test)]
                            #[path = "../../../../../../🔄️rotate-selection3d/🧪️tests/🧱️no-targets/🦀️.rs"]
                            pub mod tests_no_targets;
                        }
                        #[path = "."]
                        pub mod scale_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔍️scale-selection3d/🧪️tests/🔍️scales/🦀️.rs"]
                            pub mod tests_scales_parts_and_volume;
                            #[cfg(test)]
                            #[path = "../../../../../../🔍️scale-selection3d/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../../../../../🔍️scale-selection3d/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../../../../../🔍️scale-selection3d/🧪️tests/⏸️keeps-unit-factors/🦀️.rs"]
                            pub mod tests_keeps_unit_factors;
                            #[cfg(test)]
                            #[path = "../../../../../../🔍️scale-selection3d/🧪️tests/🧱️zero-factor/🦀️.rs"]
                            pub mod tests_zero_factor;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::capsule_dream;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::concrete_forest;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::nakagin_capsule_tower;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_mut_any_5::editor::puzzle5d::snapshot;
        #[path = "../../../../../../../../✏️editor/🧩️entities/🦀️.rs"]
        pub mod entities;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../../../../../../../👁️viewer/🦀️.rs"]
                mod component;
        pub use component::*;
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
