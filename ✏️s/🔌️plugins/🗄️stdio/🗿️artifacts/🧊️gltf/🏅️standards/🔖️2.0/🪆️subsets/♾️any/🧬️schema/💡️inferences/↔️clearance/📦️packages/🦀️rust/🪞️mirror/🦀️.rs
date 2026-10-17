//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                        #[path = "../../../🦀️.rs"]
                        pub mod clearance;
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "../../../../🧭️orientation/🦀️.rs"]
                        pub mod orientation;
                        #[path = "."]
                        pub mod roughness {
                            #[path = "."]
                            pub mod deviation_from_ideal {
                                pub use semio_s_artifact_stdio_gltf_inferences_flatness::standards::v2_0::subsets::any::schema::inferences::roughness::deviation_from_ideal::*;
                                #[cfg(test)]
                                #[path = "../../../../🌊️roughness/🎯️deviation-from-ideal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "."]
                            pub mod volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::area_volume::volume::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/📦️volume/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "../../../../📏️proportion/🦀️.rs"]
                        pub mod proportion;
                        #[path = "."]
                        pub mod mass_distribution {
                            #[path = "."]
                            pub mod centroid {
                                pub use semio_s_artifact_stdio_gltf_inferences_centroid::standards::v2_0::subsets::any::schema::inferences::mass_distribution::centroid::*;
                                #[cfg(test)]
                                #[path = "../../../../⚖️mass-distribution/🎯️centroid/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../⚖️mass-distribution/🧮️inertia-tensor/🦀️.rs"]
                            pub mod inertia_tensor;
                            #[path = "."]
                            pub mod moments_of_inertia {
                                pub use semio_s_artifact_stdio_gltf_inferences_centroid::standards::v2_0::subsets::any::schema::inferences::mass_distribution::moments_of_inertia::*;
                                #[cfg(test)]
                                #[path = "../../../../⚖️mass-distribution/🌀️moments-of-inertia/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod principal_axes {
                                pub use semio_s_artifact_stdio_gltf_inferences_centroid::standards::v2_0::subsets::any::schema::inferences::mass_distribution::principal_axes::*;
                                #[cfg(test)]
                                #[path = "../../../../⚖️mass-distribution/🧭️principal-axes/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod principal_frame {
                                pub use semio_s_artifact_stdio_gltf_inferences_centroid::standards::v2_0::subsets::any::schema::inferences::mass_distribution::principal_frame::*;
                                #[cfg(test)]
                                #[path = "../../../../⚖️mass-distribution/🖼️principal-frame/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod compactness {
                            #[path = "."]
                            pub mod compactness_index {
                                pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness::compactness_index::*;
                                #[cfg(test)]
                                #[path = "../../../../🗜️compactness/🔢️compactness-index/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod hull_fill_ratio {
                                pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness::hull_fill_ratio::*;
                                #[cfg(test)]
                                #[path = "../../../../🗜️compactness/🫙️hull-fill-ratio/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod measure {
                                pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness::measure::*;
                                #[cfg(test)]
                                #[path = "../../../../🗜️compactness/🗜️compactness/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sphericity {
                                pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness::sphericity::*;
                                #[cfg(test)]
                                #[path = "../../../../🗜️compactness/🌐️sphericity/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod surface_to_volume_ratio {
                                pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness::surface_to_volume_ratio::*;
                                #[cfg(test)]
                                #[path = "../../../../🗜️compactness/➗️surface-to-volume-ratio/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod curvature {
                            #[path = "."]
                            pub mod curvature_histogram {
                                pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature::curvature_histogram::*;
                                #[cfg(test)]
                                #[path = "../../../../🌀️curvature/📊️curvature-histogram/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod gaussian_curvature {
                                pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature::gaussian_curvature::*;
                                #[cfg(test)]
                                #[path = "../../../../🌀️curvature/🧮️gaussian-curvature/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod mean_curvature {
                                pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature::mean_curvature::*;
                                #[cfg(test)]
                                #[path = "../../../../🌀️curvature/🌀️mean-curvature/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod sharp_feature_proportion {
                                pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature::sharp_feature_proportion::*;
                                #[cfg(test)]
                                #[path = "../../../../🌀️curvature/🗡️sharp-feature-proportion/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod concavity {
                            #[path = "."]
                            pub mod concavity_index {
                                pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity::concavity_index::*;
                                #[cfg(test)]
                                #[path = "../../../../🕳️concavity/🔢️concavity-index/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod convex_hull_gap {
                                pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity::convex_hull_gap::*;
                                #[cfg(test)]
                                #[path = "../../../../🕳️concavity/📏️convex-hull-gap/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod reentrant_area {
                                pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity::reentrant_area::*;
                                #[cfg(test)]
                                #[path = "../../../../🕳️concavity/🪣️reentrant-area/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod reentrant_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity::reentrant_volume::*;
                                #[cfg(test)]
                                #[path = "../../../../🕳️concavity/📦️reentrant-volume/🧪️tests/🔬️unit/🦀️.rs"]
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
