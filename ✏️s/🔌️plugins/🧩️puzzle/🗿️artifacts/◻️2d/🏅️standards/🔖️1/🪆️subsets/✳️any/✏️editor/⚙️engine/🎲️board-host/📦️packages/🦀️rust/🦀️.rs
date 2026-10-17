//! 🎲️ board-host of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

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
                    #[path = "."]
                    pub mod graph_manifest {
                        pub use semio_s_artifact_puzzle_2d_graph_manifest::standards::v1::subsets::any::schema::graph_manifest::*;
                        pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::graph_manifest::puzzle2d_default;
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
            #[path = "../../🦀️.rs"]
            pub mod board_host;
        }
    }
}
