//! 📍️ mut-any-8 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
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
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_edge_geometry {
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests_repositions_edge_1;
                        }
                        #[path = "."]
                        pub mod change_edge_kind {
                            #[cfg(test)]
                            #[path = "../../../../../🏷️change-edge-kind/🧪️tests/🏷️kinds/🦀️.rs"]
                            pub mod tests_kinds_a_capsule_door_edge_as_a_link;
                            #[cfg(test)]
                            #[path = "../../../../../🏷️change-edge-kind/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_kinding_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../🏷️change-edge-kind/🧪️tests/🏷️rekinds-edge-1/🦀️.rs"]
                            pub mod tests_rekinds_edge_1;
                        }
                        #[path = "."]
                        pub mod change_edge_tips {
                            #[cfg(test)]
                            #[path = "../../../../../🖇️change-edge-tips/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_tipping_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../🖇️change-edge-tips/🧪️tests/🔀️swaps-edge-1-tips/🦀️.rs"]
                            pub mod tests_swaps_edge_1_tips;
                            #[cfg(test)]
                            #[path = "../../../../../🖇️change-edge-tips/🧪️tests/🖇️tips/🦀️.rs"]
                            pub mod tests_tips_a_capsule_door_edge;
                        }
                        #[path = "."]
                        pub mod change_edge_visible {
                            #[cfg(test)]
                            #[path = "../../../../../👀change-edge-visible/🧪️tests/🙈️hides/🦀️.rs"]
                            pub mod tests_hides_a_capsule_door_edge;
                            #[cfg(test)]
                            #[path = "../../../../../👀change-edge-visible/🧪️tests/🙈️hides-edge-1/🦀️.rs"]
                            pub mod tests_hides_edge_1;
                            #[cfg(test)]
                            #[path = "../../../../../👀change-edge-visible/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_hiding_an_edge_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_edge_locked {
                            #[cfg(test)]
                            #[path = "../../../../../🔐change-edge-locked/🧪️tests/🔒️locks-edge-1/🦀️.rs"]
                            pub mod tests_locks_edge_1;
                            #[cfg(test)]
                            #[path = "../../../../../🔐change-edge-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            pub mod tests_locks_the_base_to_tambour_edge;
                            #[cfg(test)]
                            #[path = "../../../../../🔐change-edge-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_locking_an_edge_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_manifest_id {
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/🆔change-manifest-id/🧪️tests/📦️repoints-manifest/🦀️.rs"]
                            pub mod tests_repoints_manifest;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/🆔change-manifest-id/🧪️tests/📦️repoints/🦀️.rs"]
                            pub mod tests_repoints_the_tower_at_its_example_manifest;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/🤝connect-kind-compatibility/🧪️tests/🤝️adds/🦀️.rs"]
                            pub mod tests_adds_handle_kind_pair;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/🤝connect-kind-compatibility/🧪️tests/🤝️admits/🦀️.rs"]
                            pub mod tests_admits_the_reverse_tambour_circular_pair;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/💔disconnect-kind-compatibility/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_withdrawing_a_pair_the_relation_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/💔disconnect-kind-compatibility/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_handle_kind_pair;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-middle-row/🦀️.rs"]
                            pub mod tests_withdraws_the_middle_row;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/💔disconnect-kind-compatibility/🧪️tests/💔️withdraws/🦀️.rs"]
                            pub mod tests_withdraws_the_tambour_rectangular_pair;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/📚replace-kind-catalogs/🧪️tests/🧱️off-rim-template/🦀️.rs"]
                            pub mod tests_off_rim_template;
                            #[cfg(test)]
                            #[path = "../../../../../../📚️catalog/📚replace-kind-catalogs/🧪️tests/🗑️clears/🦀️.rs"]
                            pub mod tests_clears_the_installed_handle_catalog;
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
        pub mod commands {
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎛️runtime/📐️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎛️runtime/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎛️runtime/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🎛️runtime/☑️set-selectable-kind/🦀️.rs"]
            pub mod set_selectable_kind;
        }
    }
}
