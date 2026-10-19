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
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🧰️wire/🦀️.rs"]
                            pub mod wire;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🧰️wire/🦀️.rs"]
                            pub mod wire;
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
