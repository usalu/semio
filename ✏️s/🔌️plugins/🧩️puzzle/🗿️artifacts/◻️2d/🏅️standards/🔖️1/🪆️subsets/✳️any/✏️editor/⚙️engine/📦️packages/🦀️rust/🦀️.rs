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
                #[path = "../../📐️layout/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path="../../📐️layout/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
