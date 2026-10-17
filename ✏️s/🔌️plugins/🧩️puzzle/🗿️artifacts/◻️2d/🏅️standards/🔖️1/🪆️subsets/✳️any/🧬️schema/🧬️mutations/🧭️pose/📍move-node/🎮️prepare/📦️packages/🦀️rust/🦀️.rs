//! 🎮️ mut-any-5 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

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
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::lookup;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "../../🦀️.rs"]
                        pub mod move_node_preparation;
                        #[path = "../../../../../🚩️flag/🎮️prepare/🦀️.rs"]
                        pub mod flag_preparation;
                        #[path = "../../../../../🔤️text/🎮️prepare/🦀️.rs"]
                        pub mod text_preparation;
                        pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::preparation::native_preparation_child;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod canonical_fields {
                            pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::mutations::canonical_fields::*;
                            #[cfg(test)]
                            #[path="../../../../../🧵️canonical/🧪️tests/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod create_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🌱create-node/🧪️tests/🧱️zero-width-node/🦀️.rs"]
                            pub mod tests_zero_width_node;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🌱create-node/🧪️tests/🌱️appends/🦀️.rs"]
                            pub mod tests_appends_a_capsule_to_the_tower;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🌱create-node/🧪️tests/🌱️appends-node-c/🦀️.rs"]
                            pub mod tests_appends_node_c;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🌱create-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_capsule_id_the_tower_already_holds;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::*;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🗑️delete-node/🧪️tests/🚫️deletes/🦀️.rs"]
                            pub mod tests_deletes_the_tambour_and_severs_its_ten_edges;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🗑️delete-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🗑️delete-node/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_node_a_and_severs_edge;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🗑️delete-node/🧪️tests/🗑️removes-middle-node/🦀️.rs"]
                            pub mod tests_removes_the_middle_node_and_severs_its_edges;
                        }
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::inverse;
                            #[cfg(test)]
                            #[path = "../../../🧪️tests/📍️moves/🦀️.rs"]
                            pub mod tests_moves_a_capsule_across_the_shaft;
                            #[cfg(test)]
                            #[path = "../../../🧪️tests/📍️moves-node-a/🦀️.rs"]
                            pub mod tests_moves_node_a;
                            #[cfg(test)]
                            #[path = "../../../🧪️tests/🚫️rejects/🦀️.rs"]
                            pub mod tests_rejects_moving_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod replace_node_geometry {
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🧊replace-node-geometry/🧪️tests/🧱️negative-radius/🦀️.rs"]
                            pub mod tests_negative_radius;
                            #[cfg(test)]
                            #[path = "../../../../../🟢️node/🧊replace-node-geometry/🧪️tests/🔳️circle/🦀️.rs"]
                            pub mod tests_circle_to_rectangle;
                        }
                    }
                }
            }
        }
    }
}
