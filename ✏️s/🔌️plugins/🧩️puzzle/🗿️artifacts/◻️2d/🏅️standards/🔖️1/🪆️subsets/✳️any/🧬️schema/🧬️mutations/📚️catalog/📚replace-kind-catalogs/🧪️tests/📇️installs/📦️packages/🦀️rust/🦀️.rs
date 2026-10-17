//! 📇️ mut-any-9 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

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
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests_installs_handle_kind_catalog;
                            #[cfg(test)]
                            #[path = "../../../📇️installs2/🦀️.rs"]
                            pub mod tests_installs_the_tower_handle_catalog;
                        }
                        #[path = "."]
                        pub mod create_target_region {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🌍create-target-region/🧪️tests/🌍️appends-region-2/🦀️.rs"]
                            pub mod tests_appends_region_2;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🌍create-target-region/🧪️tests/🌍️paints/🦀️.rs"]
                            pub mod tests_paints_a_tower_footprint;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🌍create-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_region_id_the_board_already_holds;
                        }
                        #[path = "."]
                        pub mod delete_target_region {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🪦delete-target-region/🧪️tests/🪦️removes-region-1/🦀️.rs"]
                            pub mod tests_removes_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🪦delete-target-region/🧪️tests/🪦️removes-middle-region/🦀️.rs"]
                            pub mod tests_removes_the_middle_region;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🪦delete-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod move_target_region {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🚀move-target-region/🧪️tests/🚀️slides-region-1/🦀️.rs"]
                            pub mod tests_slides_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🚀move-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_moving_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod resize_target_region {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/📐resize-target-region/🧪️tests/📐️widens-region-1/🦀️.rs"]
                            pub mod tests_widens_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/📐resize-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_resizing_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod edit_target_region_label {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🖋️edit-target-region-label/🧪️tests/🖋️renames/🦀️.rs"]
                            pub mod tests_renames_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🖋️edit-target-region-label/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_renaming_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_region_hidden {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🙈change-target-region-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                            pub mod tests_hides_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🙈change-target-region-hidden/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_hiding_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_region_locked {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🔏change-target-region-locked/🧪️tests/🔏️locks/🦀️.rs"]
                            pub mod tests_locks_region_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍️region/🔏change-target-region-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_locking_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod drag_selection {
                            #[cfg(test)]
                            #[path = "../../../../../../✋️selection/✋️drag-selection/🧪️tests/🧱️no-targets/🦀️.rs"]
                            pub mod tests_no_targets;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️selection/✋️drag-selection/🧪️tests/✋️drags-two-nodes/🦀️.rs"]
                            pub mod tests_drags_two_nodes;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️selection/✋️drag-selection/🧪️tests/🎯️drags-node-and-region/🦀️.rs"]
                            pub mod tests_drags_node_and_region;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️selection/✋️drag-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../../../../../✋️selection/✋️drag-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
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
        #[path = "../../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
