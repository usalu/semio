//! 🔂️ repetition-ratio of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod topology {
                            #[path = "../../../../🕸️topology/➰️boundary-loops/🦀️.rs"]
                            pub mod boundary_loops;
                            #[path = "../../../../🕸️topology/🧮️euler-characteristic/🦀️.rs"]
                            pub mod euler_characteristic;
                            #[path = "../../../../🕸️topology/🔢️genus/🦀️.rs"]
                            pub mod genus;
                            #[path = "../../../../🕸️topology/🥯️handles/🦀️.rs"]
                            pub mod handles;
                        }
                        #[path = "."]
                        pub mod symmetry {
                            #[path = "../../🦀️.rs"]
                            pub mod repetition_ratio;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                        #[path = "."]
                        pub mod adjacency {
                            #[path = "."]
                            pub mod connected_components {
                                pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::adjacency::connected_components::*;
                                #[cfg(test)]
                                #[path = "../../../../🤝️adjacency/🧩️connected-components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod contact_graph_degree {
                                pub use semio_s_artifact_stdio_gltf_inferences_connected_components::standards::v2_0::subsets::any::schema::inferences::adjacency::contact_graph_degree::*;
                                #[cfg(test)]
                                #[path = "../../../../🤝️adjacency/🌐️contact-graph-degree/🧪️tests/🔬️unit/🦀️.rs"]
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
