//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    #[path = "."]
                    pub mod fonts {
                        #[path = "../../../../../../🔨️modules/🔤️fonts/📏️metrics/🦀️.rs"]
                        pub mod metrics;
                        #[path = "../../../../../../🔨️modules/🔤️fonts/🔠️truetype/🦀️.rs"]
                        pub mod truetype;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../🦀️.rs"]
                        pub mod model;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../../🚪️io/🚨️error/🦀️.rs"]
                    pub mod error;
                    #[path = "../../../../../../🚪️io/🔤️text-string/🦀️.rs"]
                    pub mod text_string;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
