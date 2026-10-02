//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v3_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod geometry {
                #[path = "../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
