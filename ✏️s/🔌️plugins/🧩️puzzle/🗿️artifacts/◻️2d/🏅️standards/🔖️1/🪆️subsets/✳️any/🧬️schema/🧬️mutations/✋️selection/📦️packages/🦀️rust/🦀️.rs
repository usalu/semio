//! ✋️ mut-any-10 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
pub use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

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
                    pub mod graph_manifest {
                        pub use semio_s_artifact_puzzle_2d_graph_manifest::*;
                    }
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "."]
                        pub mod connect_handles_preparation {
                            #[path = "../../../🔌️handle/🪢️connect-handles/🎮️prepare/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::literal_id_inverse;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod disconnect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection {
                            #[cfg(test)]
                            #[path = "../../✋️drag-selection/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
                            pub mod tests_keeps_a_zero_offset;
                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/🧱️repeated-targets/🦀️.rs"]
                            pub mod tests_repeated_targets;
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/🔄️turns-two-nodes/🦀️.rs"]
                            pub mod tests_turns_two_nodes;
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/🎯️skips-the-region/🦀️.rs"]
                            pub mod tests_skips_the_region;
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../🔄️rotate-selection/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
                            pub mod tests_keeps_a_zero_angle;
                        }
                        #[path = "."]
                        pub mod scale_selection {
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/⛔️negative-factor/🦀️.rs"]
                            pub mod tests_negative_factor;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/🧱️zero-factor/🦀️.rs"]
                            pub mod tests_zero_factor;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/🔍️doubles-two-nodes/🦀️.rs"]
                            pub mod tests_doubles_two_nodes;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/🎯️halves-node-region/🦀️.rs"]
                            pub mod tests_halves_node_region;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            pub mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            pub mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "../../🔍️scale-selection/🧪️tests/⏸️keeps-a-unit-factor/🦀️.rs"]
                            pub mod tests_keeps_a_unit_factor;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
                    }
                    #[path = "../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_inferences::examples::puzzle2d::concrete_forest;
        pub use semio_s_artifact_puzzle_2d_inferences::examples::puzzle2d::nakagin_capsule_tower;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path="../../../../../✏️editor/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::*;
        }
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/🧊️document/🛍️set-active-example/🦀️.rs"]
            pub mod set_active_example;
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
