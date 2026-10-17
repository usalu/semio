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
                        pub mod connect_kind_compatibility {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_adds_grip_pair;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            #[cfg(test)]
                            #[path = "../../../../../../💔disconnect-kind-compatibility/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_grip_pair;
                            #[cfg(test)]
                            #[path = "../../../../../../💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-middle-row/🦀️.rs"]
                            pub mod tests_withdraws_the_middle_row;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[cfg(test)]
                            #[path = "../../../../../../📚replace-kind-catalogs/🧪️tests/📇️null/🦀️.rs"]
                            pub mod tests_null_catalogs_is_noop;
                        }
                        #[path = "."]
                        pub mod create_target_volume {
                            #[cfg(test)]
                            #[path = "../../../../../../🌍create-target-volume/🧪️tests/🧊️appends-volume-2/🦀️.rs"]
                            pub mod tests_appends_volume_2;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍create-target-volume/🧪️tests/🌲️paints/🦀️.rs"]
                            pub mod tests_paints_the_seed_bay;
                            #[cfg(test)]
                            #[path = "../../../../../../🌍create-target-volume/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_volume_id_the_model_already_holds;
                        }
                        #[path = "."]
                        pub mod delete_target_volume {
                            #[cfg(test)]
                            #[path = "../../../../../../🪦delete-target-volume/🧪️tests/🚫️removes-volume-1/🦀️.rs"]
                            pub mod tests_removes_volume_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🪦delete-target-volume/🧪️tests/🪦️removes-middle-volume/🦀️.rs"]
                            pub mod tests_removes_the_middle_volume;
                            #[cfg(test)]
                            #[path = "../../../../../../🪦delete-target-volume/🧪️tests/🌲️clears/🦀️.rs"]
                            pub mod tests_clears_the_seed_bay;
                            #[cfg(test)]
                            #[path = "../../../../../../🪦delete-target-volume/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod move_target_volume {
                            #[cfg(test)]
                            #[path = "../../../../../../🚀move-target-volume/🧪️tests/⬆️lifts-volume-1/🦀️.rs"]
                            pub mod tests_lifts_volume_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🚀move-target-volume/🧪️tests/🌲️shifts/🦀️.rs"]
                            pub mod tests_shifts_the_seed_bay_north;
                            #[cfg(test)]
                            #[path = "../../../../../../🚀move-target-volume/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_moving_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod rotate_target_volume {
                            #[cfg(test)]
                            #[path = "../../../../../../🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                            pub mod tests_half_turn_about_z;
                            #[cfg(test)]
                            #[path = "../../../../../../🌀rotate-target-volume/🧪️tests/🌲️squares/🦀️.rs"]
                            pub mod tests_squares_the_seed_bay;
                            #[cfg(test)]
                            #[path = "../../../../../../🌀rotate-target-volume/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_rotating_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod scale_target_volume {
                            #[cfg(test)]
                            #[path = "../../../../../../📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform/🦀️.rs"]
                            pub mod tests_per_axis_to_uniform;
                            #[cfg(test)]
                            #[path = "../../../../../../📐scale-target-volume/🧪️tests/🌲️widens-the-seed-bay/🦀️.rs"]
                            pub mod tests_widens_the_seed_bay;
                            #[cfg(test)]
                            #[path = "../../../../../../📐scale-target-volume/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_scaling_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_volume_hidden {
                            #[cfg(test)]
                            #[path = "../../../../../../🙈change-target-volume-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                            pub mod tests_hides_volume_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🙈change-target-volume-hidden/🧪️tests/🌲️hides/🦀️.rs"]
                            pub mod tests_hides_the_seed_bay;
                            #[cfg(test)]
                            #[path = "../../../../../../🙈change-target-volume-hidden/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_hiding_a_volume_the_model_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_volume_locked {
                            #[cfg(test)]
                            #[path = "../../../../../../🔐change-target-volume-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            pub mod tests_locks_volume_1;
                            #[cfg(test)]
                            #[path = "../../../../../../🔐change-target-volume-locked/🧪️tests/🌲️locks/🦀️.rs"]
                            pub mod tests_locks_the_seed_bay;
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
        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../../✏️editor/🎮️commands/☑️set-selectable-kind/🦀️.rs"]
            pub mod set_selectable_kind;
            #[path = "../../../../../../../../✏️editor/🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs"]
            pub mod set_transform_gumball_flag;
        }
    }
}
