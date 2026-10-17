//! 🕸️ topology of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod symmetry {
                            #[path = "."]
                            pub mod modularity_ratio {
                                pub use semio_s_artifact_stdio_gltf_inferences_footprint_area::standards::v2_0::subsets::any::schema::inferences::symmetry::modularity_ratio::*;
                                #[cfg(test)]
                                #[path = "../../../🪞️symmetry/🧩️modularity-ratio/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod repetition_ratio {
                                pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::symmetry::repetition_ratio::*;
                                #[cfg(test)]
                                #[path = "../../../🪞️symmetry/🔂️repetition-ratio/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod size {
                            #[path = "."]
                            pub mod overall_size {
                                pub use semio_s_artifact_stdio_gltf_inferences_footprint_area::standards::v2_0::subsets::any::schema::inferences::size::overall_size::*;
                                #[cfg(test)]
                                #[path = "../../../📦️size/📏️overall-size/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod mass_distribution {
                            #[path = "."]
                            pub mod inertia_tensor {
                                pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::mass_distribution::inertia_tensor::*;
                                #[cfg(test)]
                                #[path = "../../../⚖️mass-distribution/🧮️inertia-tensor/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod thickness {
                            #[path = "."]
                            pub mod mean_thickness {
                                pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness::mean_thickness::*;
                                #[cfg(test)]
                                #[path = "../../../↕️thickness/⚖️mean-thickness/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod minimum_thickness {
                                pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness::minimum_thickness::*;
                                #[cfg(test)]
                                #[path = "../../../↕️thickness/📉️minimum-thickness/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod thickness_distribution {
                                pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness::thickness_distribution::*;
                                #[cfg(test)]
                                #[path = "../../../↕️thickness/📊️thickness-distribution/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod thickness_variability {
                                pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness::thickness_variability::*;
                                #[cfg(test)]
                                #[path = "../../../↕️thickness/↔️thickness-variability/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "../../🦀️.rs"]
                        pub mod topology;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
