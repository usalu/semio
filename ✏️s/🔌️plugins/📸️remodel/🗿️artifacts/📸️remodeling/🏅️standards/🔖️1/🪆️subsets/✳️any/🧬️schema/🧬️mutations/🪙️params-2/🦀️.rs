//! 🪙️ remodeling parameter mutations: the eight update-*-params leaves (part 2).

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
                        pub mod update_feature_params {
                            #[path = "../🌠update-feature-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🌠update-feature-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🌠update-feature-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_sfm_params {
                            #[path = "../🧮update-sfm-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧮update-sfm-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧮update-sfm-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_mesh_params {
                            #[path = "../🕸️update-mesh-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🕸️update-mesh-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🕸️update-mesh-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_geo_params {
                            #[path = "../🌐update-geo-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🌐update-geo-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🌐update-geo-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
