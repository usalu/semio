//! ✏️ editor of `semio-s-artifact-stdio-semio-image` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_image_snapshot::io::text::snapshot;
        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_image_diff::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_image_diff::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_image_diff::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "../../../👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_image_diff::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_image_diff::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                }
            }
        }
    }
}
