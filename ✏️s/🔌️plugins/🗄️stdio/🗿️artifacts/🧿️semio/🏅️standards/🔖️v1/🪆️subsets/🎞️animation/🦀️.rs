//! 🪆️ The `animation` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_animation_schema_2::schema::*;

    pub use semio_s_artifact_stdio_semio_animation_schema::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_animation_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_animation_schema_2::schema::inferences;
    pub use semio_s_artifact_stdio_semio_animation_schema_4::schema::mutations;
}
pub use semio_s_artifact_stdio_semio_animation_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_animation_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_animation_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_animation_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_animation_examples::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_animation_examples::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_animation_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_animation_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_animation_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::windows;
        }
    }
}
