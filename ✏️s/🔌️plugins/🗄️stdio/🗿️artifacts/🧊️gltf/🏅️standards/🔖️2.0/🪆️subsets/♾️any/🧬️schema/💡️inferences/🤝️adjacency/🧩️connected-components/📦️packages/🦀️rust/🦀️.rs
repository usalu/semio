//! 🧩️ connected-components of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod adjacency {
                            #[path = "../../🦀️.rs"]
                            pub mod connected_components;
                            #[path = "../../../🌐️contact-graph-degree/🦀️.rs"]
                            pub mod contact_graph_degree;
                            #[path = "../../../🔢️number-of-contacts/🦀️.rs"]
                            pub mod number_of_contacts;
                        }
                        #[path = "."]
                        pub mod area_volume {
                            #[path = "../../../../🧱️area-volume/🤝️contact-area/🦀️.rs"]
                            pub mod contact_area;
                            #[path = "../../../../🧱️area-volume/📥️enclosed-volume/🦀️.rs"]
                            pub mod enclosed_volume;
                            #[path = "."]
                            pub mod total_area {
                                pub use semio_s_artifact_stdio_gltf_inferences_exposed_area::standards::v2_0::subsets::any::schema::inferences::area_volume::total_area::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/🧮️total-area/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod void_volume {
                                pub use semio_s_artifact_stdio_gltf_inferences_exposed_area::standards::v2_0::subsets::any::schema::inferences::area_volume::void_volume::*;
                                #[cfg(test)]
                                #[path = "../../../../🧱️area-volume/🕳️void-volume/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
