//! ✏️ editor of `semio-s-artifact-stdio-semio-animation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_animation_schema::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_animation_schema_4::schema::mutations;
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
            pub use semio_s_artifact_stdio_semio_animation_examples::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_animation_examples::editor::modes::edit::windows;
        }
    }
    pub use semio_s_artifact_stdio_semio_animation_examples::editor::edit_rules;
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
            pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::windows;
        }
    }
}
#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
}
