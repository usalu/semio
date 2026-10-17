//! 🗂️ catalog of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
pub use semio_s_plugin_puzzle_jobs as puzzle_job;
pub use semio_s_artifact_puzzle_5d_mut_any_5::{apply_puzzle5d_mutation};

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
                        pub mod drag_selection_2d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::inverse;
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
        pub use semio_s_artifact_puzzle_5d_mut_any_5::editor::puzzle5d::snapshot;
        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        #[path = "../../🦀️.rs"]
        pub mod catalog;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
        }
        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window::*;
            #[cfg(test)]
            #[path = "../../../🪟️window/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
            #[path = "../../../🪟️window/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        #[path = "."]
        pub mod precompute {
            #[path = "../../../🧠️precompute/🦀️.rs"]
            mod component;
            pub use component::*;
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::precompute::geometry;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/📡️proximity-connect/🦀️.rs"]
            pub mod proximity_connect;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod options {
                    #[path = "../../../🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs"]
                    pub mod brush;
                }
                #[path = "."]
                pub mod tools {
                    #[path = "../../../🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs"]
                            pub mod transform;
                        }
                    }
                }
            }
        }
    }
}
