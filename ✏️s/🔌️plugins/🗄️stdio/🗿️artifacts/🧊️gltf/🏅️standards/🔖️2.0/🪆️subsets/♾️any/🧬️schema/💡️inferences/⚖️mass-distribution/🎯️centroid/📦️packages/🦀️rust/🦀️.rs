//! 🎯️ centroid of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod orientation {
                            #[path = "../../../../🧭️orientation/📊️face-normal-distribution/🦀️.rs"]
                            pub mod face_normal_distribution;
                        }
                        #[path = "."]
                        pub mod mass_distribution {
                            #[path = "../../🦀️.rs"]
                            pub mod centroid;
                            #[path = "../../../🌀️moments-of-inertia/🦀️.rs"]
                            pub mod moments_of_inertia;
                            #[path = "../../../🧭️principal-axes/🦀️.rs"]
                            pub mod principal_axes;
                            #[path = "../../../🖼️principal-frame/🦀️.rs"]
                            pub mod principal_frame;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "."]
                            pub mod enclosed_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::area_volume::enclosed_volume::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/📥️enclosed-volume/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod exposed_area {
                                pub use semio_s_artifact_stdio_gltf_inferences_exposed_area::standards::v2_0::subsets::any::schema::inferences::area_volume::exposed_area::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/☀️exposed-area/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
