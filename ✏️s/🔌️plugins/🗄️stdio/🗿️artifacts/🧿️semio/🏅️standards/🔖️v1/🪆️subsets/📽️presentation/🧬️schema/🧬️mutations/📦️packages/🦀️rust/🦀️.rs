//! 🧬️ mutations of `semio-s-artifact-stdio-semio-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![recursion_limit = "256"]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
        pub use semio_s_artifact_stdio_semio_presentation_snapshot::io::text::diff;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_presentation_mut_family::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_presentation_mut_family::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioPresentationSnapshot, schema::mutations::SemioPresentationMutation);
