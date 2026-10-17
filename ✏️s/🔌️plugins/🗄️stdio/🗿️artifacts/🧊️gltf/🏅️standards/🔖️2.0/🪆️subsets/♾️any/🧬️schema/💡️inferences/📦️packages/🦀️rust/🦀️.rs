//! 💡️ `stdio.gltf` inferences: the measurement and topology modules, the inference schema and codecs and the decoded-accessor projection.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::io::binary;
                    pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::io::text;
                    pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::io::inference_projection;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::schema::inferences::*;

                        pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::adjacency;
                        pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::area_volume;
                        pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::clearance;
                        pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness;
                        pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity;
                        pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature;
                        pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::schema::inferences::dag_assembly;
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::schema::inferences::mass_distribution;
                        pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::orientation;
                        pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::proportion;
                        pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness;
                        pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::size;
                        pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry;
                        pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness;
                        pub use semio_s_artifact_stdio_gltf_inferences_topology::standards::v2_0::subsets::any::schema::inferences::topology;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
