//! 🧬️ schema of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_puzzle_2d_graph_manifest::standards::v1::subsets::any::schema::graph_manifest;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    #[path = "."]
                    pub mod lookup {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::lookup::*;
                        #[cfg(test)]
                        #[path = "../../📸️snapshot/🔎️lookup/🧪️tests/🦀️.rs"]
                        pub mod tests;
                    }
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::testkit;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
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
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            #[path = "."]
            pub mod icons {
                pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::icons::*;
                pub use semio_s_artifact_puzzle_2d_inferences::editor::puzzle2d::engine::icons::*;
                #[cfg(test)]
                #[path = "../../../✏️editor/⚙️engine/🔣️icons/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
