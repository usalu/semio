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
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../🦀️.rs"]
                    pub mod inference_projection;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../../../../../🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::adjacency;
                        #[path = "."]
                        pub mod area_volume {
                            pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::area_volume::*;
                            pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::area_volume::contact_area;
                        }
                        #[path = "."]
                        pub mod clearance {
                            pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::clearance::*;
                            pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::interference_volume;
                            pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::overlap_volume;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_compactness::standards::v2_0::subsets::any::schema::inferences::compactness;
                        pub use semio_s_artifact_stdio_gltf_inferences_concavity::standards::v2_0::subsets::any::schema::inferences::concavity;
                        pub use semio_s_artifact_stdio_gltf_inferences_curvature::standards::v2_0::subsets::any::schema::inferences::curvature;
                        #[path = "../../../../../../../🧬️schema/💡️inferences/🏗️dag-assembly/🦀️.rs"]
                        pub mod dag_assembly;
                        #[path = "."]
                        pub mod geometry_core {
                            pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../../../🧬️schema/💡️inferences/⚖️mass-distribution/🦀️.rs"]
                        pub mod mass_distribution;
                        pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::orientation;
                        pub use semio_s_artifact_stdio_gltf_inferences_clearance::standards::v2_0::subsets::any::schema::inferences::proportion;
                        #[path = "."]
                        pub mod roughness {
                            pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness::*;
                            #[path = "."]
                            pub mod irregularity {
                                pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness::irregularity::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🌊️roughness/🪨️irregularity/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod surface_waviness {
                                pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness::surface_waviness::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🌊️roughness/🌊️surface-waviness/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod normal_variation {
                                pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness::normal_variation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🌊️roughness/🧭️normal-variation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod deviation_from_smoothed_geometry {
                                pub use semio_s_artifact_stdio_gltf_inferences_roughness::standards::v2_0::subsets::any::schema::inferences::roughness::deviation_from_smoothed_geometry::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🌊️roughness/🧽️deviation-from-smoothed/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::size;
                        #[path = "."]
                        pub mod symmetry {
                            pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry::*;
                            #[path = "."]
                            pub mod rotational_symmetry_score {
                                pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry::rotational_symmetry_score::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🪞️symmetry/🔄️rotational-symmetry-score/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod reflection_symmetry_score {
                                pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry::reflection_symmetry_score::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🪞️symmetry/⚖️reflection-symmetry-score/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod rotational_symmetries {
                                pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry::rotational_symmetries::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🪞️symmetry/🔁️rotational-symmetries/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod reflection_symmetries {
                                pub use semio_s_artifact_stdio_gltf_inferences_symmetry::standards::v2_0::subsets::any::schema::inferences::symmetry::reflection_symmetries::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🪞️symmetry/🪞️reflection-symmetries/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_thickness::standards::v2_0::subsets::any::schema::inferences::thickness;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_stdio_gltf_inferences_topology::standards::v2_0::subsets::any::schema::inferences::topology::*;
                            #[path = "."]
                            pub mod holes {
                                pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::topology::holes::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🧬️schema/💡️inferences/🕸️topology/🕳️holes/🧪️tests/🔬️unit/🦀️.rs"]
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
