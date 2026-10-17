//! 🔮️ Independent artifact-owned oracle provider with only its own standards.

use semio_framework_test_fixtures::fixture_path;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ap214 {
        #[path = "."]
        pub mod reference {
            #[path = "../🏅️standards/🔖️ap214/🔮️oracles/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc1 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc2 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc3 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc4 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc5 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cc6 {
                #[path = "../🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🔮️oracles/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

/// 📂️ Reads one committed artifact file at test runtime, by its path relative to this crate's manifest directory.
#[cfg(test)]
pub(crate) fn committed(relative: &str) -> Vec<u8> {
    std::fs::read(fixture_path!(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// 📜️ Reads one committed UTF-8 artifact file at test runtime, by its path relative to this crate's manifest directory.
#[cfg(test)]
pub(crate) fn committed_text(relative: &str) -> String {
    String::from_utf8(committed(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}
