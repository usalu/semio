//! 📦️ volume of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod clearance {
                            #[path = "../../../../↔️clearance/📊️clearance-distribution/🦀️.rs"]
                            pub mod clearance_distribution;
                            #[path = "../../../../↔️clearance/🚧️interference-volume/🦀️.rs"]
                            pub mod interference_volume;
                            #[path = "../../../../↔️clearance/📏️minimum-distance-to-neighbors/🦀️.rs"]
                            pub mod minimum_distance_to_neighbors;
                            #[path = "../../../../↔️clearance/🫂️overlap-volume/🦀️.rs"]
                            pub mod overlap_volume;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "."]
                            pub mod material_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_exposed_area::standards::v2_0::subsets::any::schema::inferences::area_volume::material_volume::*;
                                #[cfg(test)]
                                #[path = "../../../🧊️material-volume/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod surface_area {
                                pub use semio_s_artifact_stdio_gltf_inferences_exposed_area::standards::v2_0::subsets::any::schema::inferences::area_volume::surface_area::*;
                                #[cfg(test)]
                                #[path = "../../../🧥️surface-area/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../🦀️.rs"]
                            pub mod volume;
                        }
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
