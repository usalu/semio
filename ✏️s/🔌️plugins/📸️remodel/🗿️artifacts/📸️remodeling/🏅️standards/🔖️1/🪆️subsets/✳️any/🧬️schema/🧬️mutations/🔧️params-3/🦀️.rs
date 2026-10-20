//! 🔧️ remodeling parameter mutations: the eight update-*-params leaves (part 3).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod update_match_params {
                            #[path = "../🪢update-match-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🪢update-match-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🪢update-match-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_motion_params {
                            #[path = "../🏎️update-motion-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🏎️update-motion-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🏎️update-motion-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
