//! 🧬️ schema of `semio-s-artifact-stdio-semio` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
}
#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod mutations {
        #[path = "../../🧬️mutations/🎞️apply-animation/🦀️.rs"]
        pub mod apply_animation;
    }
    pub use semio_s_artifact_stdio_semio_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_inferences::schema::diff;
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_inferences::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_inferences::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
