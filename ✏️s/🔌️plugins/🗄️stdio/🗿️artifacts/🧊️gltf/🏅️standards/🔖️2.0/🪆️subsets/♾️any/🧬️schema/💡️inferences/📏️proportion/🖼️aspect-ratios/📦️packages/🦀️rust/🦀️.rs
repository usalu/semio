//! 🖼️ aspect-ratios of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

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
                        pub mod proportion {
                            #[path = "../../🦀️.rs"]
                            pub mod aspect_ratios;
                            #[path = "../../../↔️elongation/🦀️.rs"]
                            pub mod elongation;
                        }
                        #[path = "."]
                        pub mod orientation {
                            #[path = "../../../../🧭️orientation/➡️main-axis-direction/🦀️.rs"]
                            pub mod main_axis_direction;
                            #[path = "../../../../🧭️orientation/🧲️orientation-consistency/🦀️.rs"]
                            pub mod orientation_consistency;
                        }
                        #[path = "."]
                        pub mod size {
                            #[path = "../../../../📦️size/🔗️characteristic-length/🦀️.rs"]
                            pub mod characteristic_length;
                        }
                        pub use semio_s_artifact_stdio_gltf_inferences_geometry_core::standards::v2_0::subsets::any::schema::inferences::geometry_core;
                    }
                    pub use semio_s_artifact_stdio_gltf_inferences_modules::standards::v2_0::subsets::any::schema::modules;
                }
            }
        }
    }
}
