//! 🪆️ The `presentation` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod io {
    pub use semio_s_artifact_stdio_semio_presentation_io::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_presentation_io::io::binary::*;
        pub use semio_s_artifact_stdio_semio_presentation_examples::io::binary::inferences;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_semio_presentation_examples::io::sqlite::*;
        pub use semio_s_artifact_stdio_semio_presentation_snapshot::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_semio_presentation_io::io::text::*;
        pub use semio_s_artifact_stdio_semio_presentation_examples::io::text::inferences;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_presentation_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::diff;
    pub use semio_s_artifact_stdio_semio_presentation_examples::schema::inferences;
    pub use semio_s_artifact_stdio_semio_presentation_mutations::schema::mutations;
}
pub use semio_s_artifact_stdio_semio_presentation_examples::examples;
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_presentation_io::editor::*;

    pub use semio_s_artifact_stdio_semio_presentation_examples::editor::presence;
    pub use semio_s_artifact_stdio_semio_presentation_examples::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_presentation_mut_family::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_presentation_mut_family::editor::modes::edit::windows;
        }
    }
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_presentation_io::viewer::*;

    pub use semio_s_artifact_stdio_semio_presentation_examples::viewer::presence;
    pub use semio_s_artifact_stdio_semio_presentation_examples::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_presentation_mutations::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_presentation_mut_family::viewer::modes::view::windows;
        }
    }
}
