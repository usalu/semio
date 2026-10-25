//! 🪆️ The `image` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_image_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_image_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_image_diff::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_image_diff::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_image_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_image_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_image_diff::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_image_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_image_diff::schema::diff;
    pub use semio_s_artifact_stdio_semio_image_schema::schema::inferences;
    pub use semio_s_artifact_stdio_semio_image_editor::schema::mutations;
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_image_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_image_diff::editor::presence;
    pub use semio_s_artifact_stdio_semio_image_diff::editor::config;
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
    pub use semio_s_artifact_stdio_semio_image_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_image_diff::viewer::presence;
    pub use semio_s_artifact_stdio_semio_image_diff::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_image_editor::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_image_diff::viewer::modes::view::windows;
        }
    }
}
