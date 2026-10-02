//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2x3 {
        #[path = "."]
        pub mod reference {
            #[path = "../🏅️standards/🔖️2x3/🔮️oracles/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cobie {
                #[path = "../🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cv20 {
                #[path = "../🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod sav {
                #[path = "../🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod v4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
