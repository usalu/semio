//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_mpeg1_layer3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

#[cfg(test)]
mod registration_tests {
    #[test]
    fn registered_target_enables_the_independent_oracles() {
        assert!(cfg!(feature = "oracles"), "the registered MP3 oracle target must enable its independent implementations");
    }
}
