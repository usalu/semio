//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod hdrl {
                #[path = "../🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
