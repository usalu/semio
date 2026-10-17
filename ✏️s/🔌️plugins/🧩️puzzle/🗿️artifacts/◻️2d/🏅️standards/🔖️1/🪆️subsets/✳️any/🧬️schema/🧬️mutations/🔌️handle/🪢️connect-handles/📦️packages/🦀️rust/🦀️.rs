//! 🪢️ mut-any-4 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

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
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::selection;
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::inverse;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::inverse;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_handles {
                            #[path = "../../../✂️disconnect-handles/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✂️disconnect-handles/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✂️disconnect-handles/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
                        pub mod change_target_region_hidden {
                            #[path = "../../../../🌍️region/🙈change-target-region-hidden/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🙈change-target-region-hidden/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🙈change-target-region-hidden/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_target_region_locked {
                            #[path = "../../../../🌍️region/🔏change-target-region-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🔏change-target-region-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🔏change-target-region-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection {
                            #[path = "../../../../✋️selection/✋️drag-selection/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✋️selection/✋️drag-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✋️selection/✋️drag-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            #[path = "../../../../✋️selection/🔄️rotate-selection/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✋️selection/🔄️rotate-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✋️selection/🔄️rotate-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection {
                            #[path = "../../../../✋️selection/🔍️scale-selection/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✋️selection/🔍️scale-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✋️selection/🔍️scale-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
