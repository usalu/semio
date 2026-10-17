//! 🤝️ adjacency of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

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
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        pub mod adjacency;
                        #[path = "../../../🧱️area-volume/🦀️.rs"]
                        pub mod area_volume;
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod topology {
                            #[path = "."]
                            pub mod boundary_loops {
                                pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::boundary_loops::*;
                                #[cfg(test)]
                                #[path = "../../../🕸️topology/➰️boundary-loops/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod euler_characteristic {
                                pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::euler_characteristic::*;
                                #[cfg(test)]
                                #[path = "../../../🕸️topology/🧮️euler-characteristic/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod genus {
                                pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::genus::*;
                                #[cfg(test)]
                                #[path = "../../../🕸️topology/🔢️genus/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod handles {
                                pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::handles::*;
                                #[cfg(test)]
                                #[path = "../../../🕸️topology/🥯️handles/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../🕸️topology/🕳️holes/🦀️.rs"]
                            pub mod holes;
                        }
                        #[path = "../../../📦️size/🦀️.rs"]
                        pub mod size;
                        #[path = "."]
                        pub mod orientation {
                            #[path = "."]
                            pub mod face_normal_distribution {
                                pub use semio_s_artifact_stdio_gltf_inferences_centroid::standards::v2_0::subsets::any::schema::inferences::orientation::face_normal_distribution::*;
                                #[cfg(test)]
                                #[path = "../../../🧭️orientation/📊️face-normal-distribution/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod main_axis_direction {
                                pub use semio_s_artifact_stdio_gltf_inferences_aspect_ratios::standards::v2_0::subsets::any::schema::inferences::orientation::main_axis_direction::*;
                                #[cfg(test)]
                                #[path = "../../../🧭️orientation/➡️main-axis-direction/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod orientation_consistency {
                                pub use semio_s_artifact_stdio_gltf_inferences_aspect_ratios::standards::v2_0::subsets::any::schema::inferences::orientation::orientation_consistency::*;
                                #[cfg(test)]
                                #[path = "../../../🧭️orientation/🧲️orientation-consistency/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod clearance {
                            #[path = "."]
                            pub mod clearance_distribution {
                                pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::clearance_distribution::*;
                                #[cfg(test)]
                                #[path = "../../../↔️clearance/📊️clearance-distribution/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod interference_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::interference_volume::*;
                                #[cfg(test)]
                                #[path = "../../../↔️clearance/🚧️interference-volume/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod minimum_distance_to_neighbors {
                                pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::minimum_distance_to_neighbors::*;
                                #[cfg(test)]
                                #[path = "../../../↔️clearance/📏️minimum-distance-to-neighbors/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod overlap_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::overlap_volume::*;
                                #[cfg(test)]
                                #[path = "../../../↔️clearance/🫂️overlap-volume/🧪️tests/🔬️unit/🦀️.rs"]
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
