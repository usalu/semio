//! ⚙️ remodeling parameter mutations: the eight update-*-params leaves.

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
                        pub mod update_ingest_params {
                            #[path = "../../../🥣update-ingest-params/🦀️.rs"]
                            mod component;
                            #[path = "../../../🥣update-ingest-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🥣update-ingest-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_dense_params {
                            #[path = "../../../🌁update-dense-params/🦀️.rs"]
                            mod component;
                            #[path = "../../../🌁update-dense-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🌁update-dense-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        pub use semio_s_artifact_remodel_remodeling_mut_params_2::standards::v1::subsets::any::schema::mutations::{update_feature_params, update_sfm_params, update_mesh_params, update_geo_params};
                        pub use semio_s_artifact_remodel_remodeling_mut_params_3::standards::v1::subsets::any::schema::mutations::{update_match_params, update_motion_params};
                    }
                }
            }
        }
    }
}
