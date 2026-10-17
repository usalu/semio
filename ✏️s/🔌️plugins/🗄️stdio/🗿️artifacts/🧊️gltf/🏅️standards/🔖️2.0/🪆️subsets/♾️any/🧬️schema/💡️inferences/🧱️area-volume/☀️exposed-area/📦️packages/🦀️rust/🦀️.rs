//! ☀️ exposed-area of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "../../🦀️.rs"]
                            pub mod exposed_area;
                            #[path = "../../../🧊️material-volume/🦀️.rs"]
                            pub mod material_volume;
                            #[path = "../../../🧥️surface-area/🦀️.rs"]
                            pub mod surface_area;
                            #[path = "../../../🧮️total-area/🦀️.rs"]
                            pub mod total_area;
                            #[path = "../../../🕳️void-volume/🦀️.rs"]
                            pub mod void_volume;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
