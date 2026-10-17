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
                        pub mod replace_trajectory {
                            #[path = "../../../🛣️replace-trajectory/🦀️.rs"]
                            mod component;
                            #[path = "../../../🛣️replace-trajectory/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🛣️replace-trajectory/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        pub use semio_s_artifact_remodel_remodeling_mut_results_2::standards::v1::subsets::any::schema::mutations::{replace_sparse, replace_mesh_result, replace_tracks, replace_qc};
                        pub use semio_s_artifact_remodel_remodeling_mut_results_3::standards::v1::subsets::any::schema::mutations::{replace_dense, replace_geo_products};
                    }
                }
            }
        }
    }
}
