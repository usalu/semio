//! 🪛️ remodeling result mutations: the replace-* result leaves and commit-reconstruction (part 3).

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
                        pub mod replace_dense {
                            #[path = "../☁️replace-dense/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../☁️replace-dense/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../☁️replace-dense/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_geo_products {
                            #[path = "../🗾replace-geo-products/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🗾replace-geo-products/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🗾replace-geo-products/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
