//! 🦶️ footprint-area of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod symmetry {
                            #[path = "../../../../🪞️symmetry/🧩️modularity-ratio/🦀️.rs"]
                            pub mod modularity_ratio;
                        }
                        #[path = "."]
                        pub mod size {
                            #[path = "../../🦀️.rs"]
                            pub mod footprint_area;
                            #[path = "../../../🧭️oriented-bounds/🦀️.rs"]
                            pub mod oriented_bounds;
                            #[path = "../../../📏️overall-size/🦀️.rs"]
                            pub mod overall_size;
                            #[path = "../../../🎯️projected-area/🦀️.rs"]
                            pub mod projected_area;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod adjacency {
                            #[path = "."]
                            pub mod number_of_contacts {
                                pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::adjacency::number_of_contacts::*;
                                #[cfg(test)]
                                #[path = "../../../../🤝️adjacency/🔢️number-of-contacts/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "."]
                            pub mod contact_area {
                                pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::area_volume::contact_area::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/🤝️contact-area/🧪️tests/🔬️unit/🦀️.rs"]
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
