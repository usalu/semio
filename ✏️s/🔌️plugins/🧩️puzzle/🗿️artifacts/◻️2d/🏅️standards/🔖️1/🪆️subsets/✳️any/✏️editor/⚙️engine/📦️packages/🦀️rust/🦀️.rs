//! ⚙️ engine of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "."]
        pub mod engine {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod icons {
                #[path = "."]
                pub mod board_metabolism_icons {
                    #[path = "../../🔣️icons/🌱️metabolism/🥁️tambour-whole/🦀️.rs"]
                    pub mod tambour_whole;
                }
            }
            #[path = "."]
            pub mod layout {
                pub use semio_s_artifact_puzzle_2d_graph_manifest::editor::puzzle2d::engine::layout::*;
                #[cfg(test)]
                #[path="../../📐️layout/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
