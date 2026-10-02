//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod v89a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
