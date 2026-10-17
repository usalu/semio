//! 🧬️ schema of `semio-s-artifact-stdio-semio-mesh` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod change_material_base_color {
            #[path = "../../🧬️mutations/🌈change-material-base/🦀️.rs"]
            mod component;
            #[path = "../../🧬️mutations/🌈change-material-base/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🧬️mutations/🌈change-material-base/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
