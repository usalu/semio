//! 📃️ flatness of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        #[path = "."]
                        pub mod size {
                            #[path = "../../../../📦️size/↔️axis-aligned-bounds/🦀️.rs"]
                            pub mod axis_aligned_bounds;
                            #[path = "../../../../📦️size/📐️bounding-box-dimensions/🦀️.rs"]
                            pub mod bounding_box_dimensions;
                        }
                        #[path = "."]
                        pub mod roughness {
                            #[path = "../../../../🌊️roughness/🎯️deviation-from-ideal/🦀️.rs"]
                            pub mod deviation_from_ideal;
                        }
                        #[path = "."]
                        pub mod proportion {
                            #[path = "../../🦀️.rs"]
                            pub mod flatness;
                            #[path = "../../../📏️slenderness/🦀️.rs"]
                            pub mod slenderness;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
