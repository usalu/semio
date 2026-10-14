//! 🧵️ remodeling result mutations: the replace-* result leaves and commit-reconstruction.

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
                        pub mod commit_reconstruction {
                            #[path = "../../../🏁commit-reconstruction/🦀️.rs"]
                            mod component;
                            #[path = "../../../🏁commit-reconstruction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🏁commit-reconstruction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_sparse {
                            #[path = "../../../⭐replace-sparse/🦀️.rs"]
                            mod component;
                            #[path = "../../../⭐replace-sparse/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../⭐replace-sparse/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_dense {
                            #[path = "../../../☁️replace-dense/🦀️.rs"]
                            mod component;
                            #[path = "../../../☁️replace-dense/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../☁️replace-dense/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_mesh_result {
                            #[path = "../../../🧱replace-mesh-result/🦀️.rs"]
                            mod component;
                            #[path = "../../../🧱replace-mesh-result/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧱replace-mesh-result/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_trajectory {
                            #[path = "../../../🛣️replace-trajectory/🦀️.rs"]
                            mod component;
                            #[path = "../../../🛣️replace-trajectory/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🛣️replace-trajectory/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_tracks {
                            #[path = "../../../🚂replace-tracks/🦀️.rs"]
                            mod component;
                            #[path = "../../../🚂replace-tracks/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚂replace-tracks/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_geo_products {
                            #[path = "../../../🗾replace-geo-products/🦀️.rs"]
                            mod component;
                            #[path = "../../../🗾replace-geo-products/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗾replace-geo-products/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_qc {
                            #[path = "../../../🧾replace-qc/🦀️.rs"]
                            mod component;
                            #[path = "../../../🧾replace-qc/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧾replace-qc/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
